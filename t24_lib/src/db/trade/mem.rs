use rusqlite::Transaction;
use crate::db::trade::Trade;

use crate::db::{Instrument, OrderType, StatusType, TriggerType};
// use crate::strategy::Strat;
macro_rules! generate_insert_params {
    ($trade:ident) => {
        (
            (
            &$trade.price,
            &$trade.dt,
            $trade.instrument as i64,
            // $trade.strategy as i64,
            $trade.lot,
            $trade.order_type as i64,
            $trade.stop_price,
            $trade.limit_price,
            $trade.trigger as i64,
            $trade.status as i64,
            $trade.pips as i64,
            $trade.parent_id as i64,
            $trade.pl as i64,
            $trade.exit_price as i64,
        )
        )
    };
}
macro_rules! generate_update_params {
    ($trade:ident, $id:ident) => {
        (
            (
            &$trade.price,
            &$trade.dt,
            $trade.instrument as i64,
            // $trade.strategy as i64,
            $trade.lot,
            $trade.order_type as i64,
            $trade.stop_price,
            $trade.limit_price,
            $trade.trigger as i64,
            $trade.status as i64,
            $trade.pips as i64,
            $trade.parent_id as i64,
            $trade.pl as i64,
            $trade.exit_price as i64,
            $id
        )
        )
    };
}

macro_rules! get_row {
    ($row:ident) => {
        (
          ($row.get(0)?, Trade {
                price: $row.get(1)?,
                dt: $row.get(2)?,
                instrument: Instrument::from_i64($row.get(3)?),
                // strategy: Strat::from_i64($row.get(4)?),
                lot: $row.get(4)?,
                order_type: OrderType::from_i64($row.get(5)?),
                stop_price: $row.get(6)?,
                limit_price: $row.get(7)?,
                trigger: TriggerType::from_i64($row.get(8)?),
                status: StatusType::from_i64($row.get(9)?),
                pips: $row.get(10)?,
                parent_id: $row.get(11)?,
                pl: $row.get(12)?,
                exit_price: $row.get(13)?,
            })
        )
    };
}



pub const DB_INIT: &str = r##"
drop table if exists trades;
CREATE TABLE if not exists trades (
    id  INTEGER PRIMARY KEY,
    price  INTEGER NOT NULL,
    dt  INTEGER NOT NULL,
    instrument INTEGER NOT NULL,
    strategy INTEGER NOT NULL,
    lot INTEGER NOT NULL,
    order_type INTEGER NOT NULL,
    stop_price INTEGER NOT NULL,
    limit_price INTEGER NOT NULL,
    trigger INTEGER NOT NULL,
    status INTEGER NOT NULL,
    pips INTEGER NOT NULL,
    parent_id INTEGER NOT NULL,
    pl INTEGER NOT NULL,
    exit_price INTEGER NOT NULL
);
CREATE INDEX trades_instrument_idx ON trades (instrument);
"##;

pub fn init(tx:&Transaction){
    tx.execute_batch(
        DB_INIT,
    ).unwrap();
}

pub fn insert_tx(
    tx: &Transaction,
    data: Trade,
) -> (i64,Trade) {
    let (trade_id, mut trade) = tx.query_row(
        &Trade::sqlite_insert_string(),
        generate_insert_params!(data),
        |row|{
            Ok(get_row!(row))
        }
    ).unwrap();
    if trade.parent_id == 0 {
        trade.parent_id = trade_id;
        update_tx(
            &tx,
            trade_id,
            trade
        );
    }
    (trade_id,trade)
}

pub fn get_last(
    tx:&Transaction,
) -> Vec<(i64, Trade)> {
    get_tx(tx, "SELECT * FROM trades ORDER BY t DESC LIMIT 1".to_string())
}

pub fn update_tx(
    tx: &Transaction,
    id: i64,
    trade: Trade,
) -> (i64,Trade) {
    tx.query_row(
        &Trade::sqlite_update_string(),
        generate_update_params!(trade,id),
        |row|{
            Ok(get_row!(row))
        }
    ).unwrap()
}

pub fn get_tx(
    tx: &Transaction,
    sql: String,
) -> Vec<(i64, Trade)> {
    let mut stmt = tx.prepare(sql.as_str()).unwrap();
    let data = stmt.query_map([], |row|  Ok(get_row!(row)))
        .unwrap()
        .collect::<Result<Vec<_>, _>>().unwrap();
    data
}

pub fn get_tx_opt(
    tx: &Transaction,
    sql: String,
) -> Result<Vec<(i64, Trade)>,rusqlite::Error> {
    let mut stmt = tx.prepare(sql.as_str()).unwrap();
    let data = stmt.query_map([], |row|  {
        let i:Result<i64,rusqlite::Error> = row.get(0);
        if i.is_err() {
            Err(rusqlite::Error::QueryReturnedNoRows)
        } else {
            Ok(get_row!(row))
        }
    })
        .unwrap()
        .collect::<Result<Vec<_>, _>>();
    data
}