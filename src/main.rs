extern crate dotenv;

use dotenv::dotenv;
use model::user::User;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use crate::model::user::SolanaUser;

mod api;
mod util;
mod model;

fn main() {
    dotenv().ok();

    let user1 = User::new("MY_PUB_KEY");
    println!("first user balance: {:?}", user1.get_balance());

    let user2 = User::new("OTHER_PUB_KEY");
    println!("second user balance: {:?}", user2.get_balance());

    // get sols - airdrop
    // user1.get_sols();

    // transfer sols from user1 to user2
    user1.transfer_sols(&user2.get_pubkey(), LAMPORTS_PER_SOL);

    println!("first user balance: {:?}", user1.get_balance());
    println!("second user balance: {:?}", user2.get_balance());
}
