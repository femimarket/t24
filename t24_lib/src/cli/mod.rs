use std::env::var;
// use near_cli_rs::commands::TopLevelCommand;
use clap::Parser;
use crate::trial::near::std::set_trial;
use crate::trial::Trial;

#[derive(Debug,Parser)] // requires `derive` feature
pub enum CargoCli {
    Trial(Trial),
}



pub fn cli() {
    // let a = TopLevelCommand::try_parse_from(vec![
    //     "near",
    //     "contract",
    //     "call-function",
    //     "as-transaction",
    //     "1.test.near",
    //     "set_trial",
    //     "json-args",
    //     "{}",
    //     "prepaid-gas",
    //     "100.0 Tgas",
    //     "attached-deposit",
    //     "10 NEAR",
    //     "sign-as",
    //     "test.near",
    //     "network-config",
    //     "local",
    //     "sign-with-keychain",
    //     "send"
    //
    // ]);
    let cli = CargoCli::parse();
    let contract_id = var("CONTRACT_ID").expect("CONTRACT_ID");
    let rpc_url = var("RCP_URL").expect("RCP_URL");
    let key_path = var("KEY_PATH").expect("KEY_PATH");
    match cli {
        CargoCli::Trial(args) => {
            set_trial(
                &rpc_url,
                &contract_id,
                &key_path,
                args
            );
        }
    }
}