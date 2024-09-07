#[cfg(feature = "std")]

use near_sdk::json_types::U64;
use near_sdk::near;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};


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

#[cfg(feature = "std")]
pub mod std {
    use near_workspaces::{AccountId, InMemorySigner,types::NearToken};
    use std::str::FromStr;
    use near_workspaces::types::SecretKey;
    use crate::near::trial::{NearTrial, Trial};
    use crate::t24_std::NearKp;

    pub fn set_trial(
        rpc_url:&str,
        contract_id:&str,
        key_path:&str,
        _: Trial,
    ){
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let sandbox = near_workspaces::custom(rpc_url).await.unwrap();
            let signer = InMemorySigner::from_file(key_path.as_ref()).unwrap();
            let outcome = sandbox.call(
                &signer,
                &AccountId::from_str(contract_id).unwrap(),
                "set_trial"
            ).args_json(NearTrial{
                trial:Trial{
                    trader: Default::default(),
                    platform_fee_taken: false,
                    liquidator_fee_taken: false,
                }
            })
                .transact()
                .await
                .unwrap();

            println!("{:?}",outcome)
        });
    }
}





