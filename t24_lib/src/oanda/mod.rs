use std::str::FromStr;
use chrono::{DateTime, Utc};
use rayon::iter::IntoParallelRefIterator;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use crate::instrument::Instrument;
use crate::t24_std::time_iter;
use crate::tick::Tick;
use rayon::iter::ParallelIterator;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OandaInstrument {
    pub instrument: String,
    pub candles: Vec<OandaCandle>,
}



#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct OandaCandle {
    pub complete: bool,
    pub bid: OandaCandlePrice,
    pub mid: OandaCandlePrice,
    pub ask: OandaCandlePrice,
    pub time: DateTime<Utc>,
    pub volume: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct OandaCandlePrice {
    pub o: String,
    pub h: String,
    pub l: String,
    pub c: String,
}

fn from_instrument(i:Instrument)->String {
    match i {
        Instrument::EurUsd => "EUR_USD".to_string(),
        _ => unimplemented!("{:?}",i)
    }
}

fn to_instrument(i:&str)->Instrument {
    match i.to_uppercase().as_str() {
        "EUR_USD" => Instrument::EurUsd,
        _ => unimplemented!("{}",i)
    }
}

pub fn fetch_hlocv_oanda(
    instrument: Instrument, // Replace Instrument with the actual type
    from: i64,
    to: i64,
    secret: String
) -> Vec<Tick> {

    let instrument_str = from_instrument(instrument);
    println!("from {}",DateTime::<Utc>::from_timestamp_millis(from).unwrap());
    println!("to {}",DateTime::<Utc>::from_timestamp_millis(to).unwrap());

    // let increase_time_by = match granularity.as_str() {
    //     "D" => 15552000000, // 15B
    //     "H1" => 1728000000, // 1B
    //     "S5" => 21600000, // 1B
    //     _ => panic!("{:?}", granularity)
    // };
    let increase_time_by = 21600000;
    let granularity = "S5".to_string();

    let urls = time_iter(from, to, increase_time_by)
        .chunks(2)
        .map(|x| {
            let f = DateTime::<Utc>::from_timestamp_millis(x[0]).unwrap().timestamp();
            let t = DateTime::<Utc>::from_timestamp_millis(x[1]).unwrap().timestamp();;
            format!(
                "https://api-fxtrade.oanda.com/v3/instruments/{instrument}/candles?granularity={granularity}&alignmentTimezone=UTC&from={from}&to={to}&smooth=false&price=BAM",
                /* Snakecase it */
                instrument = format!("{}", instrument_str.to_uppercase()),
                granularity = granularity,
                from = f,
                to = t
            )
        }).collect::<Vec<_>>();

    let client = reqwest::blocking::Client::new();

    let from_s = |x: &str| {
        Decimal::from_str(x).unwrap() * dec!(10_000)
    };

    let mut results = rayon::ThreadPoolBuilder::new().num_threads(16).build().unwrap().install(|| {
        urls.par_iter().map(|x| {
            let client = client.clone();
            let resp = client
                .get(x)
                .header("Authorization", format!("Bearer {}", secret))
                .send().
                unwrap();
            match resp.status().is_client_error() {
                true => {
                    let url = resp.url().clone();
                    println!("{} {}", resp.text().unwrap(), url);
                    panic!("ff");
                }
                false => {
                    resp.json::<OandaInstrument>().unwrap()
                }
            }
        })
            .flat_map(|x| x.candles)
            .map(|x| {
                let ask = Decimal::from_str(x.ask.c.as_str()).unwrap() * dec!(10_000);
                let bid = Decimal::from_str(x.bid.c.as_str()).unwrap() * dec!(10_000);
                let mid = Decimal::from_str(x.mid.c.as_str()).unwrap() * dec!(10_000);
                let c = Decimal::from_str(x.mid.c.as_str()).unwrap() * dec!(10_000);
                let v = Decimal::from_u64(x.volume).unwrap();
                Tick {
                    ask: ask.to_i64().unwrap(),
                    bid: bid.to_i64().unwrap(),
                    mid: mid.to_i64().unwrap(),
                    v: v.to_i64().unwrap(),
                    dt: x.time.timestamp_millis(),
                    instrument
                }
            })
            .collect::<Vec<_>>()

    });

    results
}