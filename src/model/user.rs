use solana_sdk::pubkey::Pubkey;
use crate::util::basic_helper;
use crate::api::solana_service;

pub trait SolanaUser {
    fn new(env_key: &str) -> Self;
    fn get_balance(&self) -> u64;
    fn get_sols(&self);
    fn sync_sols(&mut self);
    fn get_pubkey(&self) -> Pubkey;
    fn transfer_sols(&self, to_pubkey: &Pubkey, lamports_to_transfer: u64);
}

#[derive(Debug)]
pub struct User {
    pub_key: Pubkey,
    balance: u64,
}

impl SolanaUser for User {
    fn new(env_key: &str) -> Self {
        Self {
            balance: 0,
            pub_key: basic_helper::get_pubkey(env_key),
        }
    }

    fn get_pubkey(&self) -> Pubkey {
        self.pub_key
    }

    fn get_balance(&self) -> u64 {
        solana_service::get_balance(&self.pub_key)
    }

    fn get_sols(&self) {
        solana_service::get_sols(&self.pub_key)
    }

    fn sync_sols(&mut self) {
        self.balance = Self::get_balance(&self);
    }

    fn transfer_sols(&self, to_pubkey: &Pubkey, lamports_to_transfer: u64) {
        solana_service::transfer_sols(
            &self.pub_key,
            "MY_SECRET_KEY",
            to_pubkey,
            lamports_to_transfer,
        )
    }
}