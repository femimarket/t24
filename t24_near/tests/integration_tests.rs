use near_indexer_test_framework::*;

pub async fn hello(a:tokio::sync::mpsc::Receiver<near_indexer::StreamerMessage>){

}

#[test]
fn test_login_create_new_account() {
    while_indexer(hello, |ns| {
        let s = NearSigner::Known(ns.near_root_ai.to_string(),ns.near_root_ais.to_string()).signer()
        .unwrap();
        let d = DeployContract{
            contract: "factory.wasm",
            deploy_to: Option::from(s.account_id.to_string().as_str()),
        };
        let p = ns.near_port;

        async move  {
            let account ="bob.test.near";
            ns.create_account(None, vec![CreateAccount{
                account,
                deposit: to_near(10)
            }]).await;
            // println!("hello0 {:?}",s.account_id);

            // let resp = login_create_new_account(
            //     "http://localhost",
            //     p.to_string().as_str(),
            //         "random"
            //     )
            //     .await;
            // assert_eq!(hyper::StatusCode::OK, resp.status());
            // let b = hyper::body::to_bytes(resp).await.unwrap();
            // let v = b.to_vec();
            // let s = String::from_utf8(v).unwrap();
            // println!("{}", s);
        }
    });
}