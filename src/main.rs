extern crate dotenv;

use dotenv::dotenv;

mod util;
mod api;

fn main() {
    dotenv().ok();

    println!("Before getting sols");
    api::solana_service::get_balance();

    api::solana_service::get_sols();

    println!("After getting sols");
    api::solana_service::get_balance();
}
