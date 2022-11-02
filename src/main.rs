use eyre::Result;
mod node;
use dotenv::dotenv;
use ethers::prelude::*;
use gobbler_bindings::art_gobblers::ArtGobblersEvents;
use log::{debug, info};
use node::Node;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use webhook::client::WebhookClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Running GobblerWatch!");

    let discord = Arc::new(WebhookClient::new(&std::env::var(
        "GOBBLER_DISCORD_WEBHOOK",
    )?));
    let watch_address: Address = std::env::var("WATCH_ADDRESS")?.parse()?;
    let node = Arc::new(Node::new_local_node_from_env().await?);

    let filter = Filter::new()
        .address("0x60bb1e2aa1c9acafb4d34f71585d7e959f387769".parse::<Address>()?)
        .from_block(std::env::var("CHECKPOINT_BLOCK")?.parse::<u64>()?);

    let mut stream = node.ws_client.subscribe_logs(&filter).await?;

    while let Some(log) = stream.next().await {
        debug!("Found an ArtGobblers event!");
        let event = ArtGobblersEvents::decode_log(&log_to_raw_log(&log));

        if let Ok(event) = event {
            match event {
                ArtGobblersEvents::GobblerPurchasedFilter(gpf) => {
                    let res = discord
                        .send(|message| message.content(&format!("{gpf:#?}")))
                        .await;
                    info!("Sent GobblerPurchasedFilter: {:?}", res);
                }
                ArtGobblersEvents::TransferFilter(transfer) => {
                    if transfer.from == watch_address || transfer.to == watch_address {
                        // Hack to get around discord rate limiting
                        sleep(Duration::from_millis(600)).await;
                        let res = discord
                            .send(|message| message.content(&format!("{transfer:#?}")))
                            .await;
                        info!("Sent TransferFilter: {:?}", res);
                    }
                }
                _ => (),
            }
        }
    }

    Ok(())
}

pub fn log_to_raw_log(log: &Log) -> abi::RawLog {
    abi::RawLog {
        topics: log.topics.clone(),
        data: log.data.0.to_vec(),
    }
}
