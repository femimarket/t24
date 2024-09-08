mod gt;

use std::ops::Deref;
use std::str::FromStr;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use near_account_id::AccountId;
use near_jsonrpc_client::{JsonRpcClient, methods};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{BlockReference, Finality};
use near_primitives::views::QueryRequest;
use t24_lib::trade::Trade;
use t24_lib::db::trial::Trial;

#[derive(Default)]
pub struct State {
    trials:Vec<Trial>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let state = web::Data::new(State{
        ..Default::default()
    });
    let client = JsonRpcClient::connect("https://rpc.mainnet.near.org");

    let account_id = AccountId::from_str("lilses.near")
        .unwrap();

    methods::
    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: QueryRequest::ViewAccount { account_id },
    };

    let response = client.call(request).await.unwrap();

    if let QueryResponseKind::ViewAccount(result) = response.kind {
        println!("{:#?}", result);
    }
    HttpServer::new(move|| {
        App::new()
            .app_data(state.clone())
            .service(trial)
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}


#[post("/trial")]
async fn trial(
    trial_body:web::Json<Trial>,
) -> actix_web::Result<impl Responder> {

    Ok("st".to_string())
}