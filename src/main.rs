use eyre::Result;
mod node;
use dotenv::dotenv;
use ethers::prelude::*;
use gobbler_bindings::art_gobblers::ArtGobblersEvents;
use log::{debug, info};
use node::Node;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use webhook::client::WebhookClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Running GobblerWatch!");

    let art_gobblers = "0x60bb1e2aa1c9acafb4d34f71585d7e959f387769".parse::<Address>()?;
    let watch_address: Address = std::env::var("WATCH_ADDRESS")?.parse()?;
    let checkpoint_block = std::env::var("CHECKPOINT_BLOCK")?.parse::<u64>()?;

    let discord = Arc::new(WebhookClient::new(&std::env::var(
        "GOBBLER_DISCORD_WEBHOOK",
    )?));
    let node = Arc::new(Node::new_local_node_from_env().await?);

    let (tx, mut rx) = mpsc::channel(32);
    let tx_sync = tx.clone();
    let node_sync = node.clone();
    let tx_current = tx.clone();
    let node_current = node.clone();

    let historical = tokio::spawn(async move {
        debug!("Watching for historical events");
        let current_block = node_sync.http_client.get_block_number().await?.as_u64();
        let chunk_size: usize = 8;
        // FIXME there should be a cleaner way :(
        for block_no in (checkpoint_block..current_block).step_by(chunk_size) {
            let filter = Filter::new().address(art_gobblers).from_block(block_no);
            let filter = if block_no + (chunk_size as u64) < current_block {
                filter.to_block(block_no + chunk_size as u64)
            } else {
                filter.from_block(current_block)
            };
            debug!("Syncing {chunk_size} blocks from block: {:?}", block_no);
            for log in node_sync.http_client.get_logs(&filter).await? {
                tx_sync.send(log).await?;
            }
        }
        Ok::<(), eyre::Report>(())
    });

    let current = tokio::spawn(async move {
        debug!("Watching for current event");
        let filter = Filter::new()
            .address(art_gobblers)
            .from_block(BlockNumber::Latest);

        let mut stream = node_current.ws_client.subscribe_logs(&filter).await?;

        while let Some(event) = stream.next().await {
            tx_current.send(event).await?;
        }

        Ok::<(), eyre::Report>(())
    });

    let receiver = tokio::spawn(async move {
        while let Some(log) = rx.recv().await {
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
                    ArtGobblersEvents::GooBalanceUpdatedFilter(gbuf) => {
                        if gbuf.user == watch_address {
                            let res = discord
                                .send(|message| message.content(&format!("{gbuf:#?}")))
                                .await;
                            info!("Sent GooBalanceUpdatedFilter: {:?}", res);
                        }
                    }
                    _ => (),
                }
            }
        }
        Ok::<(), eyre::Report>(())
    });

    historical.await??;
    current.await??;
    receiver.await??;

    Ok(())
}

pub fn log_to_raw_log(log: &Log) -> abi::RawLog {
    abi::RawLog {
        topics: log.topics.clone(),
        data: log.data.0.to_vec(),
    }
}
