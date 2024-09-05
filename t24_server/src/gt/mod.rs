use actix_web::web;
pub use serde::*;
use t24_lib::db::trial::{TrialQuery, TrialService};

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub user: String,
    pub index: String,
    pub pair_index: String,
    pub leverage: String,
    pub long: bool,
    pub is_open: bool,
    pub collateral_index: String,
    pub trade_type: String,
    pub collateral_amount: String,
    pub open_price: String,
    pub tp: String,
    pub sl: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeInfo {
    pub created_block: String,
    pub tp_last_updated_block: String,
    pub sl_last_updated_block: String,
    pub max_slippage_p: String,
    pub last_oi_update_ts: u64,
    pub collateral_price_usd: String,
    pub contracts_version: String,
    pub last_pos_increase_block: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiquidationParams {
    pub max_liq_spread_p: String,
    pub start_liq_threshold_p: String,
    pub end_liq_threshold_p: String,
    pub start_leverage: String,
    pub end_leverage: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitialAccFees {
    pub acc_pair_fee: String,
    pub acc_group_fee: String,
    pub block: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllTrade {
    pub trade: Trade,
    pub trade_info: TradeInfo,
    pub liquidation_params: LiquidationParams,
    pub initial_acc_fees: InitialAccFees,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GtTradeAction {
    TradeOpenedMarket,
    TradeClosedLIQ
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GtTrade {
    pub pair: String,
    pub action: GtTradeAction,
    pub pnl: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradingVariablesRequest {
    pub all_trades: Vec<AllTrade>,
}


// fn scoped_config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/test")
//             .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
//             .route(web::head().to(HttpResponse::MethodNotAllowed)),
//     );
// }

fn pass(
    trial_query: web::Query<TrialQuery>
){
    let client = reqwest::blocking::Client::new();
    match trial_query.trial_service {
        TrialService::GainsTrade => {
            let gt_trades = client.get(format!("https://backend-arbitrum.gains.trade/personal-trading-history-table/{account}",account=trial_query.account))
                .send()
                .unwrap()
                .json::<Vec<GtTrade>>()
                .unwrap();
        }
    }

}