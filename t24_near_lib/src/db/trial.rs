use near_sdk::near;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[near(serializers = [json, borsh])]
pub struct Trial {
    pub sender_account:String,
    pub deposit_account:String,
    pub test_account:String,
    pub live_account:String,
    pub option: TrialOption
}


#[derive(Debug)]
#[near(serializers = [json, borsh])]
pub enum TrialOption {
    O10k,
    O100k
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
