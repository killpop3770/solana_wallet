use std::{fs::File, io::Read, time::Instant};

use anyhow::{Context, Result};
use bs58;
use chrono::{DateTime, Utc};
use futures::future::join_all;
use rand::Rng;
use reqwest::Client;
use serde::Deserialize;
use solana_sdk::{
    instruction::Instruction, message::Message, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::Transaction,
};
use tokio::task::JoinHandle;

const MAIN_URL: &str = "https://api.devnet.solana.com";

#[derive(Debug, Deserialize)]
struct Config {
    transmitters: Vec<TransmitterConfig>,
    receivers: Vec<String>,
    transaction_amount: u64,
}

#[derive(Debug, Deserialize)]
struct TransmitterConfig {
    private_key: String,
    address: String,
}

#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
    jsonrpc: String,
    result: T,
    id: u8,
}

#[derive(Debug, Deserialize)]
struct SendTransactionResponse {
    signature: String,
}

#[derive(Debug, Deserialize)]
struct TransactionStatusResponse {
    err: Option<String>,
    slot: u64,
    confirmation_status: Option<String>,
}

async fn load_config() -> Result<Config> {
    let mut file = File::open("config.yaml").context("Failed to open config.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Failed to read config.yaml")?;
    let config: Config = serde_yaml::from_str(&contents).context("Failed to parse config.yaml")?;
    Ok(config)
}

fn generate_unique_id() -> String {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen();
    format!("{:x}", random_number)
}

async fn send_transaction(
    client: &Client,
    transmitter_config: &TransmitterConfig,
    receiver_address: &String,
    amount: u64,
) -> Result<String> {
    let transmitter_keypair =
        Keypair::from_bytes(&bs58::decode(&transmitter_config.private_key).unwrap()).unwrap();
    let receiver_pubkey = Pubkey::from_str(receiver_address).unwrap();
    let transmitter_pubkey = Pubkey::from_str(&transmitter_config.address).unwrap();

    let instruction = Instruction::new_with_bytes(
        solana_sdk::system_program::id(),
        &solana_sdk::system_instruction::transfer(&transmitter_pubkey, &receiver_pubkey, amount).data,
        vec![],
    );

    let message = Message::new(&[instruction], Some(&transmitter_pubkey));

    let recent_blockhash = get_recent_blockhash(&client).await?;

    let transaction = Transaction::new(&[&transmitter_keypair], message, recent_blockhash);

    let params = vec![bs58::encode(bincode::serialize(&transaction).unwrap()).into_string()];

    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "sendTransaction",
        "params": params,
    });

    let response = client
        .post(MAIN_URL)
        .json(&request_body)
        .send()
        .await
        .context("Failed to send transaction request")?;

    let rpc_response: RpcResponse<SendTransactionResponse> = response
        .json()
        .await
        .context("Failed to parse sendTransaction JSON response")?;

    Ok(rpc_response.result.signature)
}

async fn get_recent_blockhash(client: &Client) -> Result<solana_sdk::hash::Hash> {
    let params: Vec<String> = vec!["confirmed".to_string()];
    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getRecentBlockhash",
          "params": params,
    });

    let response = client
        .post(MAIN_URL)
        .json(&request_body)
        .send()
        .await
        .context("Failed to send getRecentBlockhash request")?;

    let rpc_response: RpcResponse<serde_json::Value> = response
        .json()
        .await
        .context("Failed to parse getRecentBlockhash JSON response")?;

    let blockhash_string = rpc_response
        .result
        .get("value")
        .context("blockhash not found")?
        .get("blockhash")
        .context("blockhash not found in value")?
        .as_str()
        .context("blockhash is not string")?
        .to_string();

    Ok(solana_sdk::hash::Hash::from_str(&blockhash_string).unwrap())
}

async fn check_transaction_status(
    client: &Client,
    signature: String,
) -> Result<TransactionStatusResponse> {
    let params = vec![signature.clone(), "confirmed".to_string()];

    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTransaction",
          "params": params,
    });

    let response = client
        .post(MAIN_URL)
        .json(&request_body)
        .send()
        .await
        .context("Failed to send getTransaction request")?;

    let rpc_response: RpcResponse<Option<TransactionStatusResponse>> = response
        .json()
        .await
        .context("Failed to parse getTransaction JSON response")?;

    match rpc_response.result {
        Some(status) => Ok(status),
        None => anyhow::bail!("Transaction not found."),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config().await?;
    let client = Client::new();
    let amount = config.transaction_amount;

    let mut transactions: Vec<JoinHandle<Result<(String, Instant, DateTime<Utc>)>>> = Vec::new();

    for sender_config in config.transmitters.iter() {
        for recipient_address in config.receivers.iter() {
            let client = client.clone();
            let sender_config = sender_config.clone();
            let recipient_address = recipient_address.clone();
            let fut = tokio::spawn(async move {
                let start_time = Instant::now();
                let start_utc: DateTime<Utc> = Utc::now();

                let signature =
                    send_transaction(&client, &sender_config, &recipient_address, amount).await?;

                Ok((signature, start_time, start_utc))
            });
            transactions.push(fut);
        }
    }

    let transaction_results = join_all(transactions).await;

    println!("Transfers completed. Checking status:");
    let mut check_status_futures: Vec<
        JoinHandle<Result<(String, TransactionStatusResponse, std::time::Duration)>>,
    > = Vec::new();
    for transfer in transaction_results {
        match transfer {
            Ok(Ok((signature, start_time, start_utc))) => {
                let client = client.clone();
                let signature_for_check = signature.clone();

                let fut = tokio::spawn(async move {
                    let status = check_transaction_status(&client, signature_for_check).await?;
                    let duration = Instant::now().duration_since(start_time);
                    Ok((signature, status, duration))
                });
                check_status_futures.push(fut);
            }
            Ok(Err(e)) => println!("Error during transfer: {}", e),
            Err(e) => println!("Error during join: {}", e),
        }
    }

    let status_results = join_all(check_status_futures).await;

    println!("Transaction Status:");
    for status in status_results {
        match status {
            Ok(Ok((signature, status, duration))) => {
                let confirmation_status =
                    status.confirmation_status.unwrap_or("not found".to_string());
                println!(
                    "Signature: {}, Status: {}, Duration: {:?}, Slot: {}",
                    signature, confirmation_status, duration, status.slot
                );
                if let Some(err) = status.err {
                    println!("Error: {:?}", err);
                }
            }
            Ok(Err(e)) => println!("Error during check: {}", e),
            Err(e) => println!("Error during join check: {}", e),
        }
    }

    Ok(())
}
