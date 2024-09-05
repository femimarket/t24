use std::ops::Deref;
use rusqlite::{Connection, Transaction};
use crate::db;
use crate::db::{Instrument, tick};
use crate::db::tick::Tick;

macro_rules! generate_insert_params {
    ($data:ident) => {
        (
            (
                &$data.ask,
                &$data.bid,
                &$data.mid,
                &$data.v,
                &$data.dt,
                $data.instrument as i64
            )
        )
    };
}
macro_rules! generate_update_params {
    ($data:ident, $id:ident) => {
        (
            (
                &$data.ask,
                &$data.bid,
                &$data.mid,
                &$data.v,
                &$data.dt,
                $data.instrument,
                $id
            )
        )
    };
}

macro_rules! get_row {
    ($row:ident) => {
        (
          ($row.get(0)?, Tick {
                    ask: $row.get(1)?,
                    bid: $row.get(2)?,
                    mid: $row.get(3)?,
                    v: $row.get(4)?,
                    dt: $row.get(5)?,
                    instrument: Instrument::from_i64($row.get(6)?),
            })
        )
    };
}

pub const DB_INIT: &str = r##"
drop table if exists ticks;
CREATE TABLE if not exists ticks (
    id  INTEGER PRIMARY KEY,
    ask  INTEGER NOT NULL,
    bid  INTEGER NOT NULL,
    mid  INTEGER NOT NULL,
    v  INTEGER NOT NULL,
    dt  INTEGER NOT NULL,
    instrument  INTEGER NOT NULL
);
CREATE INDEX ticks_instrument_idx ON ticks (instrument);
"##;

pub const DB_INIT_BRICK: &str = r##"
drop table if exists bricks;
CREATE TABLE if not exists bricks (
    id  INTEGER PRIMARY KEY,
    ask  INTEGER NOT NULL,
    bid  INTEGER NOT NULL,
    mid  INTEGER NOT NULL,
    v  INTEGER NOT NULL,
    dt  INTEGER NOT NULL,
    instrument  INTEGER NOT NULL
);
CREATE INDEX bricks_instrument_idx ON bricks (instrument);
"##;

pub fn init(conn:&Transaction){
    conn.execute_batch(
        DB_INIT,
    ).unwrap();
}

pub fn insert22(
    instrument:String,
    data:Vec<Tick>
){
    let mut conn = db::mem();
    let tx = conn.transaction().unwrap();
    data.into_iter().for_each(|d|{
        tx.execute(
            &db::tick::Tick::sqlite_insert_string(),
            generate_insert_params!(d),
        ).unwrap();
    });
    tx.commit().unwrap();
}

pub fn insert_txs(
    tx:&Transaction,
    data:Vec<Tick>
)->Vec<(i64,Tick)>{
    data.into_iter().map(|d|{
        tx.query_row(
            &Tick::sqlite_insert_string(),
            generate_insert_params!(d),
            |row|{
                Ok(get_row!(row))
            }
        ).unwrap()
    }).collect()
}

pub fn insert_tx(
    tx:&Transaction,
    data:Tick
)->(i64,Tick){
    tx.query_row(
        &Tick::sqlite_insert_string(),
        generate_insert_params!(data),
        |row|{
            Ok(get_row!(row))
        }
    ).unwrap()
}

pub fn get_tx(
    tx:&Connection,
    sql:String
) -> Vec<(i64, Tick)> {
    let mut stmt = tx.prepare(sql.as_str(), ).unwrap();
    let data = stmt.query_map([], |row| {
        Ok((row.get(0)?, Tick {
            ask: row.get(1)?,
            bid: row.get(2)?,
            mid: row.get(3)?,
            v: row.get(4)?,
            dt: row.get(5)?,
            instrument: Instrument::from_i64(row.get(6)?),
        }))
    }).unwrap().collect::<Result<Vec<_>,_>>().unwrap();
    data
}

pub fn get_last_signal(
    instrument:String
) -> i64 {
    let conn = db::mem();

    let query = format!("SELECT s FROM prices WHERE ID = (SELECT MAX(ID) FROM prices) and instrument = '{instrument}'");
    let mut stmt = conn.prepare(&query).unwrap();
    let data = stmt.query_row([], |row| {
        Ok(row.get(0).unwrap())
    }).unwrap();
    data
}

pub fn get_last(
    tx:&Transaction,
    instrument:String
) -> Vec<(i64, Tick)> {
    get_tx(tx.deref(),format!("SELECT * FROM ticks where instrument = '{instrument}' ORDER BY t DESC LIMIT 1;"))
}

pub fn update_where_signal(
    instrument:String,
    signals:Vec<i64>,
    timestamps:Vec<i64>,
){
    let mut conn = db::mem();
    let tx = conn.transaction().unwrap();
    signals.into_iter().enumerate().for_each(|(i,d)|{
        tx.execute(
            "Update ticks set s = ?1 where t = ?2 and instrument = ?3",
            (&d, &timestamps[i], &instrument),
        ).unwrap();
    });
    tx.commit().unwrap();
}