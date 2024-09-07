use serde::{Deserialize, Serialize};


use crate::db::{OrderType, StatusType, TriggerType};
use crate::instrument::Instrument;

// use crate::strategy::Strat;
#[derive(
    Debug,
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct Trade {
    pub price: i64,
    pub dt: i64,
    pub instrument: Instrument,
    // pub strategy: Strat,
    pub lot: i64,
    pub order_type: OrderType,
    pub stop_price: i64,
    pub limit_price: i64,
    pub trigger: TriggerType,
    pub status: StatusType,
    pub pips: i64,
    pub parent_id:i64,
    pub pl:i64,
    pub exit_price:i64,
}

impl Trade {


    pub fn double_limit_stop_order(
        limit: i64,
        sl: i64,
        lot: i64,
        dt: i64,
        instrument: Instrument,
        // strategy: Strat,
    ) -> Self {
        Trade {
            price: 0,
            dt,
            instrument,
            order_type: OrderType::DoubleLimitStop,
            stop_price: sl,
            trigger: TriggerType::CancelPending,
            limit_price: limit,
            lot,
            // strategy,
            status: StatusType::Pending,
            pips: 0,
            parent_id: 0,
            pl:0,
            exit_price:0,
        }
    }
    pub fn market_order(
        price: i64,
        lot: i64,
        dt: i64,
        instrument: Instrument,
        // strategy: Strat,
    ) -> Self {
        Trade {
            price,
            dt,
            instrument,
            order_type: OrderType::Market,
            stop_price: 0,
            trigger: TriggerType::Nothing,
            limit_price: 0,
            lot,
            // strategy,
            status: StatusType::Open,
            pips: 0,
            parent_id: 0,
            pl:0,
            exit_price:0
        }
    }

    pub fn close_order(
        price: i64,
        lot: i64,
        dt: i64,
        instrument: Instrument,
        // strategy: Strat,
        parent_id:i64,
        exit_price:i64,
        pips:i64,
        pl:i64
    ) -> Self {
        Trade {
            price,
            dt,
            instrument,
            order_type: OrderType::Market,
            stop_price: 0,
            trigger: TriggerType::Nothing,
            limit_price: 0,
            lot,
            // strategy,
            status: StatusType::Closed,
            pips,
            parent_id,
            pl,
            exit_price
        }
    }

    pub fn partial_close_order(
        price: i64,
        lot: i64,
        dt: i64,
        instrument: Instrument,
        // strategy: Strat,
        parent_id:i64,
        pips:i64,
        pl:i64,
    ) -> Self {
        Trade {
            price,
            dt,
            instrument,
            order_type: OrderType::Market,
            stop_price: 0,
            trigger: TriggerType::Nothing,
            limit_price: 0,
            lot,
            // strategy,
            status: StatusType::PartialClosed,
            pips,
            parent_id,
            pl,
            exit_price:0
        }
    }
}

