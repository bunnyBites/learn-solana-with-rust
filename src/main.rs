use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer};

fn main() {
    let wallet = Keypair::new(); // generate a new keypair (public key + secret key)
    let pubkey = Signer::pubkey(&wallet); // get the public key
    let rpc_url = String::from("http://127.0.0.1:8899"); // JSON RPC URL
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // check the available sols for our wallet (with public key) - before airdroping
    match client.get_balance(&pubkey) {
        Ok(balance) => {
            println!("SOLs Before airdropping -> {}", balance / LAMPORTS_PER_SOL);
        },
        Err(_) => println!("Error getting balance"),
    };

    // get sols (this are used for transactions)
    match client.request_airdrop(&pubkey, LAMPORTS_PER_SOL) {
        Ok(sig) => loop {
            if let Ok(confirmed) = client.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(_) => println!("Error requesting airdrop"),
    };

    // check the available sols for our wallet (with public key) - after airdroping
    match client.get_balance(&pubkey) {
        Ok(balance) => {
            println!("SOLs After airdropping -> {}", balance / LAMPORTS_PER_SOL);
        },
        Err(_) => println!("Error getting balance"),
    };
}
