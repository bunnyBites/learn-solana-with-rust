use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use crate::util::basic_helper;
use crate::api::solana_service::{self, get_balance};

pub trait SolanaUser {
    fn new(env_key: &str) -> Self;
    fn get_balance(&self) -> u64;
    fn get_sols(&mut self);
    fn sync_sols(&mut self);
    fn get_pubkey(&self) -> Pubkey;
    fn transfer_sols(&mut self, to_pubkey: &Pubkey, lamports_to_transfer: u64);
}

#[derive(Debug)]
pub struct User {
    pub_key: Pubkey,
    balance: u64,
}

impl SolanaUser for User {
    fn new(env_key: &str) -> Self {
        let pub_key = basic_helper::get_pubkey(env_key);

        Self {
            balance: get_balance(&pub_key),
            pub_key,
        }
    }

    fn get_pubkey(&self) -> Pubkey {
        self.pub_key
    }

    fn get_balance(&self) -> u64 {
        solana_service::get_balance(&self.pub_key)
    }

    fn get_sols(&mut self) {
        solana_service::get_sols(self)
    }

    fn sync_sols(&mut self) {
        self.balance = Self::get_balance(&self);
    }

    fn transfer_sols(&mut self, to_pubkey: &Pubkey, lamports_to_transfer: u64) {
        // 1 Sol = 1_000_000_000 lamports
        if self.get_balance() < (lamports_to_transfer / LAMPORTS_PER_SOL) {
            panic!("User doesn't have enough balance");
        }

        solana_service::transfer_sols(
            self,
            "MY_SECRET_KEY",
            to_pubkey,
            lamports_to_transfer,
        )
    }
}