use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use crate::db::Instrument;

const TABLE:&str = "ticks";



pub fn get_jan1_str() -> String{
    let jan1 = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(1,1,1).unwrap(),
        NaiveTime::from_hms_opt(0,0,0).unwrap()
    ).and_utc().timestamp_millis();
    format!("select * from {TABLE} where t >= '{jan1}'")
}

#[repr(C)]
#[derive(Debug,Clone,Copy,Ord, PartialOrd, Eq, PartialEq,Serialize,Deserialize)]
pub struct Tick {
    pub ask: i64,
    pub bid: i64,
    pub mid: i64,
    pub v: i64,
    pub dt: i64,
    pub instrument: Instrument
}


