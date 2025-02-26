use std::str::FromStr;
use std::time;
use web3::{
    futures::{self, StreamExt},
    types::{BlockNumber, FilterBuilder, Log, H160, H256},
};

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let web3 = web3::Web3::new(web3::transports::Http::new(
        "https://bsc-dataseed.binance.org/",
    )?);

    let contract_address: H160 =
        H160::from_str("0x0eD7e52944161450477ee417DE9Cd3a859b14fD0").unwrap();

    let signature_event_swap: H256 =
        H256::from_str("0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822")
            .unwrap();

    let signature_event_transfer: H256 =
        H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
            .unwrap();

    let filter = FilterBuilder::default()
        .address(vec![contract_address])
        .topics(
            Some(vec![signature_event_swap, signature_event_transfer]),
            None,
            None,
            None,
        )
        .from_block(BlockNumber::Earliest)
        .to_block(BlockNumber::Latest)
        .build();

    let filter = web3.eth_filter().create_logs_filter(filter).await?;

    let logs_stream = filter.stream(time::Duration::from_millis(100));
    futures::pin_mut!(logs_stream);

    while let Some(result_log) = logs_stream.next().await {
        let log: Log = result_log.unwrap();
        println!("----------------------------------------");
        println!("Block Number: {}", log.block_number.unwrap());
        println!("Event Address: {:?}", log.topics[0]);
        println!("From Address: {}", log.topics[1]);
        println!("To Address: {}", log.topics[2]);
        println!("Data: {}", format!("0x{}", hex::encode(log.data.0)));
    }
    Ok(())
}
