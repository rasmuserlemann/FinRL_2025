use bitcoincore_rpc::{Auth, Client, RpcApi};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use std::env;

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Read environment variables
    let rpc_user = env::var("BTC_RPC_USER").expect("BTC_RPC_USER not set");
    let rpc_password = env::var("BTC_RPC_PASSWORD").expect("BTC_RPC_PASSWORD not set");
    let rpc_url = env::var("BTC_RPC_URL").expect("BTC_RPC_URL not set");

    // Create an RPC client
    let rpc = Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).expect("Failed to create RPC client");

    // Get the current block height
    let block_count = rpc.get_block_count().expect("Failed to get block count");

    // Calculate the block height 24 hours ago
    let now = Utc::now();
    let twenty_four_hours_ago = now - Duration::hours(24);
    let mut start_block_height = block_count;
    let mut start_block_time = now;

    // Find the block height 24 hours ago
    while start_block_time > twenty_four_hours_ago && start_block_height > 0 {
        let block_hash = rpc.get_block_hash(start_block_height).expect("Failed to get block hash");
        let block = rpc.get_block(&block_hash).expect("Failed to get block");
        start_block_time = chrono::DateTime::from_timestamp(block.header.time as i64, 0).unwrap();
        if start_block_time <= twenty_four_hours_ago {
            break;
        }
        start_block_height -= 1;
    }

    // Calculate the number of transactions in the last 24 hours
    let mut total_transactions = 0;
    for height in start_block_height..=block_count {
        let block_hash = rpc.get_block_hash(height).expect("Failed to get block hash");
        let block = rpc.get_block(&block_hash).expect("Failed to get block");
        total_transactions += block.txdata.len();
    }

    println!("Total transactions in the last 24 hours: {}", total_transactions);
}