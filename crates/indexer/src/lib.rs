// Index the blocks, save them to db.

pub mod error;

use reqwest::Client;
use serde_json::Value;
use tokio;

use common::env;
use error::*;

#[tokio::main]
pub async fn run() -> Result<()> {
    let gateway: String = env::var("GATEWAY")?;

    let address: String = env::var("CONTRACT_ADDRESS")?;

    let c_deployed_at = env::var_as::<u64>("CONTRACT_DEPLOYED_AT")?;

    let topic_transfer = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

    let client = Client::new();

    let dataset_height: u64 = client
        .get(format!("{}/height", gateway))
        .send()
        .await?
        .text()
        .await?
        .trim()
        .parse()
        .expect("Failed to parse dataset height");

    if dataset_height < c_deployed_at {
        return Err(Error::InvalidDatasetRange);
    }

    let current_block = c_deployed_at;

    let worker_url = client
        .get(format!("{}/{}/worker", gateway, current_block))
        .send()
        .await?
        .text()
        .await?
        .trim()
        .to_string();

    let query_payload = serde_json::json!({
        "logs": [
        {
            "address": [address],
            "topic0": [topic_transfer],
            "transaction": true
        }
        ],
        "fields": {
            "block": {
                "gasUsed": true
            },
            "log": {
                "topics": true,
                "data": true
            }
        },
        "fromBlock": current_block,
        "includeAllBlocks": true,
    });

    let response = client
        .post(&worker_url)
        .json(&query_payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    dbg!(&response);

    Ok(())
}
