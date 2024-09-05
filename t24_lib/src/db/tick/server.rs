// use postgres::{GenericClient, Transaction};
// use rayon::iter::IntoParallelRefIterator;
// use rayon::iter::ParallelIterator;
//
// use crate::db::Instrument;
// use crate::db::tick::Tick;
//
// macro_rules! generate_insert_params {
//     ($data:ident) => {
//         &[
//                 &$data.ask,
//                 &$data.bid,
//                 &$data.mid,
//                 &$data.v,
//                 &$data.dt,
//                 &($data.instrument as i64)
//             ]
//     };
// }
//
// macro_rules! get_row {
//     ($row:ident) => {
//         (
//           ($row.get::<usize, i64>(0), Tick {
//                     ask: $row.get(1),
//                     bid: $row.get(2),
//                     mid: $row.get(3),
//                     v: $row.get(4),
//                     dt: $row.get(5),
//                     instrument: Instrument::from_i64($row.get(6)),
//             })
//         )
//     };
// }
//
// pub const DB_INIT: &str = r##"
// drop table if exists ticks;
// CREATE TABLE if not exists ticks (
//     id  BIGSERIAL PRIMARY KEY,
//     ask  BIGINT NOT NULL,
//     bid  BIGINT NOT NULL,
//     mid  BIGINT NOT NULL,
//     v  BIGINT NOT NULL,
//     dt  BIGINT NOT NULL,
//     instrument  BIGINT NOT NULL
// );
// CREATE INDEX ticks_instrument_idx ON ticks (instrument);
// "##;
//
// pub fn init(tx:&mut postgres::Transaction){
//     tx.batch_execute(
//         DB_INIT,
//     ).unwrap();
// }
//
//
// pub fn insert_tx(
//     tx: &mut Transaction,
//     data:Tick
// ) ->(i64,Tick){
//     let row = tx.query_one(
//         &Tick::postgres_insert_string(),
//         generate_insert_params!(data)
//     ).unwrap();
//     let a = get_row!(row);
//     a
// }
//
// // pub fn insert_txs_par(
// //     mut tx: postgres::Client,
// //     data:Vec<Tick>
// // ) ->Vec<(i64,Tick)>{
// //     data.par_iter().map(|data|{
// //         let row = tx.query_one(
// //             &Tick::postgres_insert_string(),
// //             generate_insert_params!(data)
// //         ).unwrap();
// //         let a = get_row!(row);
// //         a
// //     }).collect::<Vec<_>>()
// // }
//
// pub fn get_txs(
//     tx: &mut Transaction,
//     sql:String
// ) -> Vec<(i64, Tick)> {
//     let stmt = tx.prepare(sql.as_str(), ).unwrap();
//     tx.query(&stmt, &[]).unwrap().into_iter().map(|row| {
//         get_row!(row)
//     }).collect()
// }