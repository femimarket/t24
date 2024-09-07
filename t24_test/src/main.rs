use std::env::var;
use t24_lib::near::trial::{std::set_trial, TradingPlatform, Trial};


pub const CONTRACT_ID:&str = "t24.test.near";
pub const RCP_URL:&str = "http://localhost:3030";
pub const KEY_PATH:&str = "/home/u/.near/localnet/validator_key.json";

fn main() {
    set_trial(
        RCP_URL,
        CONTRACT_ID,
        KEY_PATH,
        Trial{
            trader: "lilses.near".to_string(),
            platform_fee_taken: false,
            liquidator_fee_taken: false,
        }
    );
}