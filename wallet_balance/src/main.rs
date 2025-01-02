use std::fs::File;
use std::io::Read;
use futures::future;
use serde::Deserialize;
use anyhow::{Result, Context};
use reqwest::{Client,  Response};

#[derive(Debug, Deserialize)]
struct Config {
    wallets: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct RpcResponse<T> {
    jsonrpc: String,
    result: T,
    id: u8,
}

#[derive(Debug, Deserialize)]
struct BalanceResponse {
    value: u64,
}

async fn get_balance(client: &Client, wallet_address: &String) -> Result<(String, u64)> {

    let params = serde_json::json!([wallet_address]);

    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBalance",
        "params": params,
    });

    let response: Response = client.post("https://api.devnet.solana.com")
        .json(&request_body)
        .send()
        .await
        .context("Failed to send request")?;

    let rpc_response: RpcResponse<BalanceResponse> = response
        .json()
        .await
        .context("Failed to parse JSON response")?;

    Ok((wallet_address.clone(), rpc_response.result.value))
}

async fn load_config() -> Result<Config> {
    let mut file = File::open("config.yaml").context("Failed to open config.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).context("Failed to read config.yaml")?;
    let config: Config = serde_yml::from_str(&contents).context("Failed to parse config.yaml")?;
    Ok(config)
}

#[tokio::main]
async fn main() -> Result<()> {
    
    let config = load_config().await?;
    let client = Client::new();

    let balance_futs: Vec<_> = config.wallets
    .iter()
    .map(|wallet| get_balance(&client, wallet))
    .collect();

    let balances = future::join_all(balance_futs)
        .await;

    for balance in balances {
        match balance {
            Ok((address, bal)) => println!("Balance for wallet {}: {} SOL", address, bal as f64 / 1000000000.0),
            Err(err) => println!("Failed to get balance: {}", err),
        }
    }

    Ok(())
}