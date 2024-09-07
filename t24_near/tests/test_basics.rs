use std::fmt::format;
use near_sdk::{AccountId, near};
use near_workspaces::InMemorySigner;
use near_workspaces::types::{KeyType, SecretKey};
use serde_json::json;
use t24_near_lib::db::trial::{NearTrial, TradingPlatform, Trial, TrialOption};

pub struct Helo {
    pub a: String,
}

#[near(serializers = [json, borsh])]
struct MyBorshSerializableStruct {
    trial: Trial,
}

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::custom("").await?;
    // let contract_wasm = near_workspaces::compile_project("./").await?;

    let wasm = include_bytes!("/home/u/Documents/RustOver/t24/target/wasm32-unknown-unknown/release/t24_near.wasm");
    let contract = sandbox.dev_deploy(wasm).await?;
InMemorySigner::from_file("".parse().unwrap());
    sandbox.call()
    let dev_pk = sandbox.dev_create_account().await?;
    let pk = sandbox.dev_create_account().await?;
    let deposit_pk = sandbox.dev_create_account().await?;
    let test_pk = sandbox.dev_create_account().await?;
    let live_pk = sandbox.dev_create_account().await?;



    let root = sandbox.root_account().unwrap();




println!("{:?}",deposit_sk.id());


    let outcome = pk
        .call(contract.id(), "set_trial")
        .args_json(NearTrial{
            trial:Trial{
                pk: pk.id().to_string(),
                deposit_pk: deposit_pk.id().to_string(),
                trading_id: test_pk.id().to_string(),
                live_trading_pk: live_pk.id().to_string(),
                trading_platform: TradingPlatform::GainsTrade,
            }
        })
        .transact()
        .await?;
    println!("{:?}", outcome);
    // assert!(outcome.is_success());
    //
    // let user_message_outcome = contract
    //     .view("get_greeting")
    //     .args_json(json!({}))
    //     .await?;
    // assert_eq!(user_message_outcome.json::<String>()?, "Hello World!");

    Ok(())
}
