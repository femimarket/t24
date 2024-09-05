use std::time::Duration;
use rusqlite::{Connection, DatabaseName, Transaction};
use rusqlite::backup::{Backup, Progress};
use serde::{Deserialize, Serialize};
use t24_macros::EnumI64Derive;
use crate::home;

pub mod brick;
pub mod account;
pub mod tick;
pub mod trade;
pub mod trial;

const DB_NAME:&str = "t24";

#[repr(C)]
#[derive(Hash,Debug, Clone, Copy, Serialize, Deserialize, EnumI64Derive, Eq, PartialEq, Ord, PartialOrd)]
pub enum Instrument {
    AudCad,
    AudChf,
    AudHkd,
    AudJpy,
    AudNzd,
    AudSgd,
    AudUsd,
    CadChf,
    CadHkd,
    CadJpy,
    CadSgd,
    ChfHkd,
    ChfJpy,
    EurAud,
    EurCad,
    EurChf,
    EurGbp,
    EurHkd,
    EurJpy,
    EurNzd,
    EurSgd,
    EurUsd,
    GbpAud,
    GbpCad,
    GbpChf,
    GbpHkd,
    GbpJpy,
    GbpNzd,
    GbpSgd,
    GbpUsd,
    HkdJpy,
    NzdCad,
    NzdChf,
    NzdHkd,
    NzdJpy,
    NzdSgd,
    NzdUsd,
    SgdChf,
    SgdHkd,
    SgdJpy,
    UsdCad,
    UsdChf,
    UsdHkd,
    UsdJpy,
    UsdSgd,
    Jpy,
    NonFarmUsPayroll,
    Usd,
    Eur
}

impl Instrument {
    pub fn pip_i64(&self)->i64{
        match self {
            Instrument::EurUsd => 1,
            Instrument::UsdJpy => 100,
            _ => panic!()
        }
    }
    pub fn q(&self){
        let (base,quote) = format!("{:?}",self).split_at(3);
    }
}





pub fn init_tx(conn:&Transaction){
    conn.execute_batch(
        account::DB_INIT,
    ).unwrap();

    conn.execute(
        "drop table if exists bricks",
        (),
    ).unwrap();

    conn.execute(
        "CREATE TABLE if not exists bricks (
            id  INTEGER PRIMARY KEY,
            tick  INTEGER NOT NULL,
            v  INTEGER NOT NULL default 0,
            t  INTEGER NOT NULL,
            instrument  TEXT NOT NULL
        )",
        (),
    ).unwrap();
}

pub fn mem() -> Connection {
    Connection::open_in_memory().unwrap()
}



fn tick_file()->String{
    format!("{home}/trade24/ticks.sqlite",home=home())
}

pub fn mem_fs() -> Connection {
    let home = std::env::var("HOME").unwrap();
    let path = format!("{home}/trade24/db.sqlite");
    Connection::open(path).unwrap()
}

pub fn mem_ticks_fs() -> Connection {
    Connection::open(tick_file()).unwrap()
}

pub fn mem_export_ticks(conn:&Connection){
    conn.backup(
        DatabaseName::Main,
        tick_file(),
        None
    ).unwrap();
}

pub fn mem_export_default(conn:&Connection){
    let home = std::env::var("HOME").unwrap();
    let path = format!("{home}/trade24/db.sqlite");
    conn.backup(
        DatabaseName::Main,
        path,
        None
    ).unwrap();
}

pub fn mem_import(conn:&Connection, path:String){
    conn.backup(
        DatabaseName::Main,
        path,
        None
    ).unwrap();
}

pub fn mem_import_ticks(conn: &mut Connection){
    conn.restore(
        DatabaseName::Main,
        tick_file(),
        None::<fn(Progress)>
    ).unwrap();
}

pub const NON_FARM_PAYROLL_US: &str = "NON_FARM_PAYROLL_US";
pub const EUR_USD: &str = "EUR_USD";
pub const EUR_USD_RENKO_100: &str = "EUR_USD_RENKO_100";

#[derive(Hash,Debug, Clone, Copy, Serialize, Deserialize,EnumI64Derive, Eq, PartialEq, Ord, PartialOrd)]

pub enum OrderType {
    Market,
    LimitStop,
    DoubleLimitStop,
}

#[derive(Hash,Debug, Clone, Copy, Serialize, Deserialize,EnumI64Derive, Eq, PartialEq, Ord, PartialOrd)]

pub enum TriggerType {
    Nothing,
    CancelPending
}

#[derive(Hash,Debug, Clone, Copy, Serialize, Deserialize,EnumI64Derive, Eq, PartialEq, Ord, PartialOrd)]

pub enum StatusType {
    Pending,
    Open,
    Cancelled,
    Closed,
    PartialClosed,
    Filled
}

