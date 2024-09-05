use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use log::info;
use rayon::iter::IntoParallelRefIterator;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use crate::db::Instrument;
use crate::db::tick::Tick;
use crate::db::trade::Trade;

pub const BUY: i64 = 1;
pub const SELL: i64 = -1;

pub mod db;

pub mod server;

pub fn units(signal:i64) ->i64{
    if signal == BUY {
        1
    } else {
        -1
    }
}

pub fn home()->String{
    std::env::var("HOME").unwrap()
}

pub fn ticks_json_path()->String{
    format!("{}/trade24/ticks.json",home())
}

pub fn ticks_bin_path()->String{
    format!("{}/trade24/ticks.bin",home())
}

pub fn date_from_str(date:&str)->chrono::DateTime<chrono::Utc>{
    chrono::Utc.from_utc_datetime(
        &chrono::NaiveDateTime::new(chrono::NaiveDate::parse_from_str(date, "%d%m%y").unwrap(), chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    )
}

pub fn date_from_str_with(date:&str,format:&str)->chrono::DateTime<chrono::Utc>{
    chrono::Utc.from_utc_datetime(
        &chrono::NaiveDateTime::new(chrono::NaiveDate::parse_from_str(date, format).unwrap(), chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap())
    )
}

pub fn trade_log(
    trade: db::trade::Trade,
    trade_id: i64,
    balance: i64,
    dt: i64,
) -> String {
    // Assuming insert_id_to_value is a function that returns a Value
    let trade_value = insert_id_to_value(trade, "id", trade_id);

    // Creating the log as a JSON object
    let log = Value::Object(Map::from_iter([
        (
            "time".to_string(),
            Value::String(DateTime::from_timestamp_millis(dt).unwrap().to_rfc3339())
        ),
        (
            "trade".to_string(),
            trade_value
        ),
        (
            "balance".to_string(),
            Value::Number(Number::from(balance))
        )
    ]));

    // Convert the log to a string and return it
    log.to_string()
}

fn insert_id_to_value<T: serde::Serialize>(item: T, id_key: &str, id_value: impl Into<Value>) -> Value {
    let mut value = serde_json::to_value(item).unwrap();
    let map = value.as_object_mut().and_then(|x| {
        x.insert(id_key.to_string(), id_value.into());
        Some(x)
    }).unwrap();
    Value::Object(map.clone())
}


// pub fn bt_run(
//     ticks:Vec<Tick>,
//     tick_fn:OnTickFn
// )->Vec<Trade>{
//     let mut hash_map_ticks = HashMap::<i64,Vec<Tick>>::new();
//     let mut trades:Vec<Trade> = vec![];
//     ticks.iter().for_each(|x|{
//         let a = hash_map_ticks.entry(x.dt).or_insert(vec![]);
//         a.push(*x);
//     });
//     hash_map_ticks.iter().for_each(|(dt,tick)|{
//         run::<()>(
//             tick.clone(),
//             &mut trades,
//             (),
//         )
//     });
//     trades
// }

pub fn run<T>(
    ticks: Vec<Tick>,
    mut trades:&mut Vec<Trade>,
    data:&mut T,
    tick_fn:fn(&[Tick],&mut Vec<Trade>, &mut T)
) {
    tick_fn(
        &ticks,
        trades,
        data
    );
}

#[tracing::instrument]
pub fn read_ticks() ->Vec<Tick>{
    let mut f = BufReader::new(File::open(ticks_bin_path()).unwrap());
    let mut s:Vec<Tick> = bincode::serde::decode_from_std_read(&mut f, bincode::config::standard()).unwrap();
    s.sort_by_key(|x|x.dt);
    // let s = serde_json::from_slice::<Vec<Tick>>(&std::fs::read(ticks_json_path()).unwrap()).unwrap();
    s
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Trade2 {
    pub date:i64,
    pub equity:i64,
}

pub fn post_bt(
    trades:Vec<Trade>,
)->Vec<Trade2>{
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://localhost:8081/bt")
        .json(&trades)
        .send()
        .unwrap()
        .json::<Vec<Trade2>>()
        .unwrap();
    res
}