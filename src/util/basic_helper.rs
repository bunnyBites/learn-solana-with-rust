use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;
use std::env;

pub fn get_pubkey(env_key: &str) -> Pubkey {
    Pubkey::from_str(env::var(env_key)
    .expect("Error finding the public key")
        .as_str())
    .expect("Error getting the public key")
}

pub fn get_keypair(env_key_for_secret: &str) -> Keypair {
    let secret_key = env::var(env_key_for_secret)
        .expect("Error finding secret key");

    Keypair::from_base58_string(&secret_key)
}

pub fn get_client() -> RpcClient {
    let rpc_url = String::from("http://127.0.0.1:8899"); // JSON RPC URL
    RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed())
}

pub fn prepare_instruction(
    from_pubkey: &Pubkey,
    to_pubkey: &Pubkey,
    lamports_to_send: u64,
) -> Instruction {
    system_instruction::transfer(
        from_pubkey,
        to_pubkey,
        lamports_to_send,
    )
}

pub fn prepare_transaction(
    ix: Instruction,
    payer: &Pubkey,
    keypair: Keypair,
    client: &RpcClient,
) -> Transaction {
    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get latest blockhash.");


    //Putting the transfer sol instruction into a transaction
    let txn = Transaction::new_signed_with_payer(
        &[ix],
        Some(payer),
        &[&keypair],
        recent_blockhash,
    );

    txn
}