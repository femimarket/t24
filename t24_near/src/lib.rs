use std::str::FromStr;
// Find all our documentation at https://docs.near.org
use near_sdk::{AccountId, BorshStorageKey, env, log, near, NearToken, PanicOnDefault, Promise, require};
use near_sdk::env::promise_batch_action_create_account;
use near_sdk::json_types::U64;
use near_sdk::store::{LookupMap, LookupSet};
use t24_lib::trade::Trade;
use t24_lib::instrument::Instrument;
use t24_lib::tick::Tick;
use t24_lib::trial::Trial;

// Define the contract structure
#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    test_accounts: LookupMap<U64,Trial>,
    accounts: LookupMap<U64,Trial>,
    ticks: LookupMap<Instrument,Tick>,
    trades: LookupMap<(AccountId,Instrument),Trade>,
    test_trades: LookupMap<(AccountId,Instrument),Trade>,
    trial_liquidation_proofs: LookupMap<Instrument,Tick>,
    last_test_trader_id: U64,
    owner: AccountId
}

#[near]
#[derive(BorshStorageKey)]
pub enum Prefix {
    TestAccount,
    Account,
    Tick,
    Trade,
    TestTrade,
    TrialLiquidationProof
}

const TRIAL_FEE:NearToken = NearToken::from_near(100);
const PLATFORM_TRIAL_FEE:NearToken = NearToken::from_near(80);
const LIQUIDATOR_TRIAL_FEE:NearToken = NearToken::from_near(10);

// Define the default, which automatically initializes the contract
// impl Default for Contract {
//     fn default() -> Self {
//         Self {
//             trials: LookupMap::new(Prefix::Trial),
//             last_trial_id: Default::default(),
//             owner: AccountId::from_str("near").unwrap(),
//         }
//     }
// }

// Implement the contract structure
#[near]
impl Contract {

    #[init]
    pub fn init(owner: AccountId) -> Self {
        Self {
            test_accounts: LookupMap::new(Prefix::TestAccount),
            accounts: LookupMap::new(Prefix::Account),
            ticks: LookupMap::new(Prefix::Tick),
            test_trades: LookupMap::new(Prefix::TestTrade),
            trades: LookupMap::new(Prefix::Trade),
            trial_liquidation_proofs: LookupMap::new(Prefix::TrialLiquidationProof),
            last_test_trader_id: Default::default(),
            owner,
        }
    }

    // pub fn platform_withdraw(&mut self, trial_id:U64){
    //     let trial = self.trials.get_mut(&trial_id).unwrap();
    //     require!(trial.platform_fee_taken == false);
    //     trial.platform_fee_taken = true;
    //     Promise::new(self.owner.clone()).transfer(PLATFORM_TRIAL_FEE);
    // }
    pub fn liquidate(&mut self, tick: Tick) {
        require!(env::signer_account_id() == self.owner);
        self.ticks.insert(tick.instrument,tick);
    }

    pub fn set_tick(&mut self, tick: Tick) {
        require!(env::signer_account_id() == self.owner);
        self.ticks.insert(tick.instrument,tick);
    }

    pub fn set_test_trade(&mut self, trade: Trade, trader:AccountId, trader_id:U64) {
        require!(env::signer_account_id() == trader);
        if let Some(account) = self.test_accounts.get(&trader_id) {
            require!(env::signer_account_id() == account.trader);
            require!(account.liquidator_fee_taken == false);

            let trade_id = (trader,trade.instrument);
            self.trades.set(trade_id,Some(trade));
        }
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    #[payable]
    pub fn set_test_account(&mut self, _t: Trial) {
        require!(env::attached_deposit() == TRIAL_FEE);
        let mut trial_id = self.last_test_trader_id;
        trial_id.0 += 1;
        // trial.time = env::block_timestamp_ms().to_string();
        self.test_accounts.insert(trial_id, Trial{
            trader:env::signer_account_id().to_string(),
            liquidator_fee_taken:false
        });

        Promise::new(self.owner.clone()).transfer(PLATFORM_TRIAL_FEE);
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
