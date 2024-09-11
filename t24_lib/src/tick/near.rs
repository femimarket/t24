#[cfg(feature = "std")]
pub mod std {
    use std::str::FromStr;
    use chrono::Utc;
    use near_sdk::AccountId;
    use near_workspaces::InMemorySigner;
    use crate::instrument::Instrument;
    use crate::oanda::fetch_hlocv_oanda;
    use crate::tick::NearTick;

    pub fn set_tick(
        rpc_url:&str,
        contract_id:&str,
        key_path:&str,
        secret:&str,
        instrument:Instrument,
    ){
        let now = Utc::now().timestamp_millis();
        let ticks = fetch_hlocv_oanda(
            instrument,
            now,
            now,
            secret.to_string()
        );

        let signer = InMemorySigner::from_file(key_path.as_ref()).unwrap();

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            for tick in ticks {
                let sandbox = near_workspaces::custom(rpc_url).await.unwrap();
                let outcome = sandbox.call(
                    &signer,
                    &AccountId::from_str(contract_id).unwrap(),
                    "set_tick"
                ).args_json(NearTick{
                    tick
                })
                    .transact()
                    .await
                    .unwrap();
                println!("{:?}",outcome);
            }
        });
    }
}


