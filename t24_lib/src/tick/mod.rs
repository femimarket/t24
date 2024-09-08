mod prelude;
pub mod server;
pub mod mem;
pub mod near;

use near_sdk::near;
pub use prelude::*;

#[derive(Debug,)]
#[near(serializers = [json, borsh])]
pub struct NearTick {
    pub tick: Tick
}