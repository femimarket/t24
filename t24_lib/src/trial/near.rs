#[cfg(feature = "std")]
pub mod std {
    use near_workspaces::{AccountId, InMemorySigner};
    use std::str::FromStr;
    use crate::trial::{NearTrial, Trial};

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