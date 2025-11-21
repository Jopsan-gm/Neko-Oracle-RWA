#![no_std]
use soroban_sdk::{self, contracttype};

mod collateralized;
mod error;
mod index_types;
mod stability_pool;
mod storage;
pub mod token;

pub use error::Error;

#[contracttype]
pub struct PriceData {
    pub price: i128,    //asset price at given point in time
    pub timestamp: u64, //recording timestamp
}

// Import RWA Oracle WASM for our RWA assets (Treasury Bonds, Real Estate, etc.)
pub mod rwa_oracle {
    soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/rwa_oracle.wasm");
}

mod test;
