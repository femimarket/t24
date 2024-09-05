use rusqlite::Transaction;
use crate::db;
use crate::db::tick::Tick;



pub fn insert(
    tx:&Transaction,
    x: Tick
){
    // tx.execute(
    //     "INSERT INTO bricks (tick,t,instrument) VALUES (?1,?2,?3)",
    //     (&x.mid, &x.dt, instrument),
    // ).unwrap();
}

pub fn get_last_n_bricks() -> Vec<i64> {
    let conn = db::mem();
    let mut stmt = conn.prepare("select tick from bricks order by t desc limit 4").unwrap();
    let data = stmt.query_map([], |row| {
        let price:i64 = row.get(0).unwrap();
        Ok(price)
    }).unwrap().collect::<Result<Vec<_>,_>>().unwrap();
    data
}