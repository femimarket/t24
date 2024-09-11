#[cfg(feature = "std")]
pub mod std {
    use std::thread::sleep;
    use std::time::Duration;

    use rayon::iter::IntoParallelRefIterator;

    use crate::instrument::Instrument;
    use crate::t24_std::{contract_id, key_path, oanda_secret, rpc_url};
    use crate::tick::near;

    fn set_tick_service(
        instruments:Vec<Instrument>
    ) {
        loop {
            instruments.par_iter().for_each(|instrument|{
                near::std::set_tick(
                    &rpc_url(),
                    &contract_id(),
                    &key_path(),
                    &oanda_secret(),
                    instrument
                );
            });
            sleep(Duration::from_secs(1));
        }
    }
}

