use solana_sdk::native_token::LAMPORTS_PER_SOL;
use crate::util::basic_helper;

pub fn get_balance() {
    let client = basic_helper::get_client();
    let pubkey = &basic_helper::get_pubkey();

    match client.get_balance(pubkey) {
        Ok(balance) => {
            println!("SOLs present -> {}", balance / LAMPORTS_PER_SOL);
        },
        Err(_) => println!("Error getting balance"),
    };
}

pub fn get_sols() {
    let client = basic_helper::get_client();
    let pubkey = basic_helper::get_pubkey();

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
}