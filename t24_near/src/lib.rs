// Find all our documentation at https://docs.near.org
use near_sdk::{AccountId, BorshStorageKey, env, log, near, NearToken, require};
use near_sdk::json_types::U64;
use near_sdk::store::{LookupMap, LookupSet};
use t24_near_lib::db::trial::Trial;

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    trials: LookupMap<U64,Trial>,
    last_trial_id: U64,
}

#[near]
#[derive(BorshStorageKey)]
pub enum Prefix {
    Trial
}


// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            trials: LookupMap::new(Prefix::Trial),
            last_trial_id: Default::default(),
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {
    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_trial(&mut self, trial: Trial) {
        let mut trial_id = self.last_trial_id;
        trial_id.0 += 1;
        self.trials.insert(trial_id,trial);
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello");
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy");
    }
}
