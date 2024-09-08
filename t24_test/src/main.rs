use std::env::var;
use t24_lib::instrument::Instrument;
use t24_lib::t24_std::oanda_secret;
use t24_lib::tick::near::std::set_tick;
use t24_lib::trial::near::std::set_trial;
use t24_lib::trial::Trial;

pub const CONTRACT_ID:&str = "t24.test.near";
pub const RCP_URL:&str = "http://localhost:3030";
pub const KEY_PATH:&str = "/home/u/.near/localnet/validator_key.json";

fn main() {
    // set_trial(
    //     RCP_URL,
    //     CONTRACT_ID,
    //     KEY_PATH,
    //     Trial{
    //         trader: "lilses.near".to_string(),
    //         liquidator_fee_taken: false,
    //     }
    // );

    set_tick(
        Instrument::EurUsd,
        &oanda_secret(),
        RCP_URL,
        CONTRACT_ID,
        KEY_PATH
    );
}