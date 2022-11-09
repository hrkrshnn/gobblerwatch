use ethers::prelude::*;
use ethers::providers::Ws;
use eyre::Result;
use log::*;
use std::{convert::TryFrom, sync::Arc, time::Duration};

#[derive(Debug, Clone)]
pub struct Node {
    pub http_client: Arc<Provider<Http>>,
    pub ws_client: Arc<Provider<Ws>>,
    pub http_endpoint: String,
    pub ws_endpoint: String,
}

impl Node {
    /// Reads from env variables `ETH_RPC_URL` and `ETH_WS_URL`
    /// - ETH_RPC_URL: json-rpc over http
    /// - ETH_WS_URL: json-rpc over ws
    pub async fn new_local_node_from_env() -> Result<Self> {
        let http_endpoint = std::env::var("ETH_RPC_URL")?;
        let ws_endpoint = std::env::var("ETH_WS_URL")?;
        info!("http_endpoint: {:?}", http_endpoint);
        info!("ws_endpoint: {:?}", ws_endpoint);

        // Polling duration of 10 second
        let http_client = Provider::<Http>::try_from(http_endpoint.clone())?
            .interval(Duration::from_millis(10000u64));

        let ws = Ws::connect(ws_endpoint.clone()).await?;
        // Polling duration of 10 second
        let ws_client = Provider::new(ws).interval(Duration::from_millis(10000));

        let http_client = Arc::new(http_client);
        let ws_client = Arc::new(ws_client);

        Ok(Node {
            http_client,
            ws_client,
            http_endpoint,
            ws_endpoint,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sanity_check_endpoint() -> Result<()> {
        let node: Node = Node::new_local_node_from_env().await?;

        assert_eq!(node.http_client.get_chainid().await?, U256::from(1));
        assert_eq!(node.ws_client.get_chainid().await?, U256::from(1));
        Ok(())
    }
}
