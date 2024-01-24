use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::env;

pub fn get_pubkey(env_key: &str) -> Pubkey {
    Pubkey::from_str(env::var(env_key)
    .expect("Error finding the public key")
        .as_str())
    .expect("Error getting the public key")
}

pub fn get_client() -> RpcClient {
    let rpc_url = String::from("http://127.0.0.1:8899"); // JSON RPC URL
    RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed())
}
