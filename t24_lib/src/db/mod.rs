use rusqlite::{Connection, DatabaseName};
use rusqlite::backup::Progress;
use serde::{Deserialize, Serialize};
use t24_macros::EnumI64Derive;
use crate::t24_std::home;

pub mod brick;
pub mod trial;

const DB_NAME:&str = "t24";


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



