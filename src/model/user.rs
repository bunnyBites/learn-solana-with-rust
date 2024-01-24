use solana_sdk::pubkey::Pubkey;
use crate::util::basic_helper;
use crate::api::solana_service;

pub trait SolanaUser {
    fn new(env_key: &str) -> Self;
    fn get_balance(&self) -> u64;
    fn get_sols(&self);
}

#[derive(Debug)]
pub struct User {
    pub_key: Pubkey,
}

impl SolanaUser for User {
    fn new(env_key: &str) -> Self {
        Self {
            pub_key: basic_helper::get_pubkey(env_key)
        }
    }

    fn get_balance(&self) -> u64 {
        solana_service::get_balance(&self.pub_key)
    }

    fn get_sols(&self) {
        solana_service::get_sols(&self.pub_key)
    }

}