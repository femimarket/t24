use rusqlite::Transaction;
use serde::{Deserialize, Serialize};
use t24_macros::SqliteUpdateStringDerive;
use crate::db;


macro_rules! generate_insert_params {
    ($data:ident) => {
        (
            (&$data.balance, &$data.dt, &$data.price)
        )
    };
}
macro_rules! generate_update_params {
    ($data:ident, $id:ident) => {
        (
            (&$data.balance, &$data.dt, &$data.price, $id)
        )
    };
}
pub fn insert_tx(
    tx: &Transaction,
    data: Account,
) {
    tx.execute(
        &Account::sqlite_insert_string(),
        generate_insert_params!(data),
    ).unwrap();
}

pub fn update_tx(
    tx: &Transaction,
    id:i64,
    data: Account,
) {
    tx.execute(
        &Account::sqlite_update_string(),
        generate_update_params!(data, id),
    ).unwrap();
}

pub fn get_tx(
    tx:&Transaction,
    sql: String
) -> Vec<(i64, Account)> {
    let mut stmt = tx.prepare(sql.as_str()).unwrap();
    let data = stmt.query_map([], |row| {
        Ok((row.get(0)?, Account {
            balance: row.get(1)?,
            dt: row.get(2)?,
            price: row.get(3)?,
        }))
    }).unwrap().collect::<Result<Vec<_>, _>>().unwrap();
    data
}

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
    SqliteUpdateStringDerive
)]
pub struct Account {
    pub balance: i64,
    pub dt: i64,
    pub price: i64,
}

pub const DB_INIT: &str = r##"
drop table if exists accounts;
CREATE TABLE if not exists accounts (
    id  INTEGER PRIMARY KEY,
    balance  INTEGER NOT NULL,
    dt  INTEGER NOT NULL
);
"##;

