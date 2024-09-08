use std::fmt::format;
use near_sdk::{AccountId, near};
use near_workspaces::InMemorySigner;
use near_workspaces::types::{KeyType, SecretKey};
use serde_json::json;
use t24_lib::instrument::Instrument;
use t24_lib::t24_std::oanda_secret;
use t24_lib::tick::near::std::set_tick;

pub const CONTRACT_ID:&str = "t24.test.near";
pub const RCP_URL:&str = "http://localhost:3030";
pub const KEY_PATH:&str = "/home/u/.near/localnet/validator_key.json";

#[test]
fn e2e_test() -> Result<(), Box<dyn std::error::Error>> {
    set_tick(
        Instrument::EurUsd,
        &oanda_secret(),
        RCP_URL,
        CONTRACT_ID,
        KEY_PATH
    );
    Ok(())
}
