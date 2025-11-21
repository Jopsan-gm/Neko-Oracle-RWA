use crate::collateralized::CDPStatus;
use soroban_sdk::{Address, contractevent};

#[contractevent(topics = ["cdp"])]
pub struct CDP {
    #[topic]
    pub id: Address,
    pub xlm_deposited: i128,
    pub asset_lent: i128,
    pub status: CDPStatus,
    pub ledger: u32,
    pub timestamp: u64,
    pub accrued_interest: i128,
    pub interest_paid: i128,
    pub last_interest_time: u64,
}

#[contractevent(topics = ["stake_position"])]
pub struct StakePosition {
    #[topic]
    pub id: Address,
    pub rwa_deposit: i128,
    pub product_constant: i128,
    pub compounded_constant: i128,
    pub rewards_claimed: i128,
    pub epoch: u64,
    pub ledger: u32,
    pub timestamp: u64,
}

#[contractevent(topics = ["liquidation"])]
pub struct Liquidation {
    #[topic]
    pub cdp_id: Address,
    pub collateral_liquidated: i128,
    pub principal_repaid: i128,
    pub accrued_interest_repaid: i128,
    pub collateral_applied_to_interest: i128,
    pub collateralization_ratio: u32,
    pub xlm_price: i128,
    pub rwa_price: i128,
    pub ledger: u32,
    pub timestamp: u64,
}

#[contractevent(topics = ["mintrwa"], data_format = "single-value")]
pub struct MintRWA {
    #[topic]
    pub to: Address,
    pub amount: i128,
}

#[contractevent(topics = ["burnrwa"], data_format = "single-value")]
pub struct BurnRWA {
    #[topic]
    pub from: Address,
    pub amount: i128,
}
