// use anyhow::{Context, Result};
// use bs58;
// use futures_util::StreamExt;
// use serde::Deserialize;
// use solana_client::{
//     client_error::ClientError,
//     nonblocking::rpc_client::RpcClient,
// };
// use solana_sdk::{
//     instruction::Instruction, message::Message, pubkey::Pubkey, signature::Keypair,
//     signer::Signer, system_instruction, transaction::Transaction,
// };
// use std::{str::FromStr};
// use tokio::time::{sleep, Duration};
// use tonic::{
//     metadata::MetadataValue,
//     transport::{Channel, ClientTlsConfig},
//     Request,
// };
// 
mod geyser {
    tonic::include_proto!("geyser");
}
// 
// #[derive(Debug, Deserialize, Clone)]
// struct Config {
//     transmitter_private_key: String,
//     transmitter_address: String,
//     receiver_address: String,
//     transaction_amount: u64,
// }
// 
// async fn load_config() -> Result<Config> {
//     let f = std::fs::File::open("config.yaml").context("Failed to open config.yaml")?;
//     let config: Config = serde_yml::from_reader(f).context("Failed to parse config.yaml")?;
//     Ok(config)
// }
// 
// async fn create_keypair(config: &Config) -> Result<Keypair> {
//     let private_key_bytes = bs58::decode(&config.transmitter_private_key)
//         .into_vec()
//         .context("Failed to decode private key from Base58")?;
// 
//     let keypair = Keypair::from_bytes(&private_key_bytes)
//         .context("Failed to create Keypair from bytes")?;
// 
//     Ok(keypair)
// }
// 
// 
// async fn send_transaction(client: &RpcClient, config: &Config, keypair: &Keypair) -> Result<String> {
//     let receiver_pubkey = Pubkey::from_str(&config.receiver_address).context("Failed to create receiver pubkey")?;
//     let from_pubkey = Pubkey::from_str(&config.transmitter_address).context("Failed to create transmitter pubkey")?;
//     let recent_blockhash = client
//         .get_latest_blockhash()
//         .await
//         .context("Failed to get recent blockhash")?;
// 
// 
//     let instruction = system_instruction::transfer(
//         &from_pubkey,
//         &receiver_pubkey,
//         config.transaction_amount,
//     );
// 
//     let message = Message::new(&[instruction], Some(&from_pubkey));
//     let transaction = Transaction::new(&[keypair], message, recent_blockhash);
//     let signature = client
//         .send_transaction(&transaction)
//         .await
//         .context("Failed to send transaction")?;
// 
//     println!("Transaction sent: {}", signature);
//     Ok(signature.to_string())
// }


// #[tokio::main]
// async
fn main() {//-> Result<()> {
    // let config = load_config().await?;
    // let keypair = create_keypair(&config).await?;
    // let rpc_url = "https://api.devnet.solana.com"; // Replace with your desired RPC URL
    // let client = RpcClient::new(rpc_url.parse()?);
    // 
    // let server_address = "grpc.ny.shyft.to";
    // let server_port = 443;
    // 
    // let token = "881e801b-7dc1-4b3e-b5c7-bccd41552961";
    // 
    // let tls_config = ClientTlsConfig::new();
    // let channel = Channel::from_shared(format!("https://{}:{}", server_address, server_port))?
    //     .tls_config(tls_config)?
    //     .connect()
    //     .await?;
    // 
    // let mut geyser_client = geyser::geyser_client::GeyserClient::new(channel);
    // 
    // let mut request = Request::new(geyser::SubscribeRequest {
    //     filters: vec![geyser::Filter {
    //         filter_type: 0,
    //         filter: "{}".to_string(),
    //     }],
    // });
    // 
    // let token_meta =  MetadataValue::from_str(token)?;
    // 
    // request.metadata_mut().insert("authorization", token_meta);
    // 
    // let mut stream = geyser_client
    //     .subscribe(request)
    //     .await?
    //     .into_inner();
    // 
    // 
    // println!("Subscribed to Geyser stream. Waiting for new blocks...");
    // 
    // while let Some(response) = stream.next().await {
    //     match response {
    //         Ok(res) => {
    //             if let Some(block) = res.block {
    // 
    //                 println!(
    //                     "Received new block: slot={}, blockhash={}",
    //                     block.slot,
    //                     block.blockhash
    //                 );
    //                 match send_transaction(&client, &config, &keypair).await {
    //                     Ok(signature) => {
    //                         println!("Successfully send transaction: {}", signature);
    //                     },
    //                     Err(e) => {
    //                         eprintln!("Error sending transaction: {}", e);
    //                     }
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             eprintln!("Error from Geyser stream: {}", e);
    //         }
    //     }
    // }
    //
    //
    // Ok(())
}