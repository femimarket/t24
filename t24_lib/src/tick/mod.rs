pub mod server;
pub mod near;

use near_sdk::near;
use crate::instrument::Instrument;

#[derive(Debug,Clone,Copy,Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "std", derive(clap::Parser))]
#[cfg_attr(feature = "std", command(version, about, long_about = None))]
#[near(serializers = [json, borsh])]
pub struct Tick {
    pub ask: i64,
    pub bid: i64,
    pub mid: i64,
    pub v: i64,
    pub dt: i64,
    pub instrument: Instrument
}

#[derive(Debug,)]
#[near(serializers = [json, borsh])]
pub struct NearTick {
    pub tick: Tick
}