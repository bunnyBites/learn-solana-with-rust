use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer};
use crate::util::basic_helper;

#[allow(dead_code)]
// generate new wallet and get its public key
pub fn generate_new_wallet() -> Pubkey {
    let wallet = Keypair::new();
    Signer::pubkey(&wallet)
}

// getting sol balance
pub fn get_balance(pub_key: &Pubkey) -> u64 {
    let client = basic_helper::get_client();

    match client.get_balance(pub_key) {
        Ok(balance) => {
            balance / LAMPORTS_PER_SOL
        },
        Err(_) => {
            println!("Error getting balance");
            0
        },
    }
}

// airdropping sols
pub fn get_sols(pub_key: &Pubkey) {
    let client = basic_helper::get_client();

    // get sols (this are used for transactions)
    match client.request_airdrop(&pub_key, LAMPORTS_PER_SOL) {
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
}