pub mod near;

use serde_repr::{Deserialize_repr, Serialize_repr};
use near_sdk::near;

/// Start t24 trial
#[derive(Debug,Clone)]
#[cfg_attr(feature = "std", derive(clap::Parser))]
#[cfg_attr(feature = "std", command(version, about, long_about = None))]
#[near(serializers = [json, borsh])]
pub struct Trial {
    pub trader:String,
    pub liquidator_fee_taken:bool,
}

#[derive(Debug,)]
#[near(serializers = [json, borsh])]
pub struct NearTrial {
    pub trial: Trial
}

#[derive(Debug,Serialize_repr, Deserialize_repr)]
#[near(serializers = [borsh])]
#[repr(u8)]
pub enum TrialOption {
    O10k,
    O100k
}

#[derive(Debug,Serialize_repr, Deserialize_repr,Copy,Clone)]
#[cfg_attr(feature = "std", derive(clap::ValueEnum))]
#[near(serializers = [borsh])]
#[repr(u8)]
pub enum TradingPlatform {
    GainsTrade,
}

#[derive(Debug)]
#[near(serializers = [json, borsh])]
pub struct TrialQuery {
    pub account: String,
    pub trial_service: TrialService
}

#[derive(Debug)]
#[near(serializers = [json, borsh])]
pub enum TrialService {
    GainsTrade
}