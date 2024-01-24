extern crate dotenv;

use dotenv::dotenv;
use model::user::User;
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
}
