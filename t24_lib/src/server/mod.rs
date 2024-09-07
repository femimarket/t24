use crate::db::trade::Trade;
// use crate::Trade2;
//
// pub fn post_trial(
//     trades:Vec<Trade>,
// )->Vec<Trade2>{
//     let client = reqwest::blocking::Client::new();
//     let res = client.post("http://localhost:8081/bt")
//         .json(&trades)
//         .send()
//         .unwrap()
//         .json::<Vec<Trade2>>()
//         .unwrap();
//     res
// }