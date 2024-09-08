

pub mod near;

#[cfg(feature = "std")]
pub mod server;
#[cfg(feature = "std")]
pub mod cli;
#[cfg(feature = "std")]
pub mod db;

pub mod tick;
pub mod instrument;
pub mod trade;

#[cfg(feature = "std")]
pub mod oanda;

pub mod trial;

#[cfg(feature = "std")]
pub mod t24_std {
    use std::cell::OnceCell;
    use std::fs::File;
    use std::io::BufReader;
    use chrono::TimeZone;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use crate::tick::Tick;
    use crate::trade::Trade;

    const OANDA_SECRET: OnceCell<String> = OnceCell::new();


    pub fn oanda_secret() -> String {
        OANDA_SECRET.get_or_init(|| {
            std::env::var("OANDA_SECRET").unwrap()
        }).to_string()
    }


    pub const BUY: i64 = 1;
    pub const SELL: i64 = -1;

    #[derive(Serialize,Deserialize)]
    pub struct NearKp {
        pub account_id:String,
        pub public_key: String,
        pub secret_key: String,
    }


    pub fn time_iter(from:i64,to:i64,increase_by:i64) -> Vec<i64>{
        let mut start = 0;
        let mut end = 0;
        let mut results = vec![];
        while end < to {
            if start == 0 && end == 0 {
                start = from;
                end = start+increase_by;
            } else {
                start = end;
                end = end + increase_by;
            }

            if end >= to {
                end = to
            }
            results.push(start);
            results.push(end);
        }
        results
    }

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

    pub fn near_home()->String{
        format!("{}/.near",home())
    }

    pub const NEAR_LOCALNET:&str = "localnet";

    pub fn near_localnet()->String{
        format!("{}/localnet",near_localnet())
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

    // pub fn trade_log(
    //     trade: super::db::trade::Trade,
    //     trade_id: i64,
    //     balance: insert_id_to_value,
    //     dt: i64,
    // ) -> String {
    //     // Assuming insert_id_to_value is a function that returns a Value
    //     let trade_value = insert_id_to_value(trade, "id", trade_id);
    //
    //     // Creating the log as a JSON object
    //     let log = Value::Object(Map::from_iter([
    //         (
    //             "time".to_string(),
    //             Value::String(DateTime::from_timestamp_millis(dt).unwrap().to_rfc3339())
    //         ),
    //         (
    //             "trade".to_string(),
    //             trade_value
    //         ),
    //         (
    //             "balance".to_string(),
    //             Value::Number(Number::from(balance))
    //         )
    //     ]));
    //
    //     // Convert the log to a string and return it
    //     log.to_string()
    // }

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
}
