//! A simple t24_zk_program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the t24_zk_program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use alloy_sol_types::sol;
use t24_lib::tick::Tick;
use t24_lib::trade::{backtest, Trade};

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32 n;
        uint32 a;
        uint32 b;
    }
}

/// Compute the n'th fibonacci number (wrapping around on overflows), using normal Rust code.
pub fn prove_liquidation(trades:Vec<Trade>,max_loss:i64) -> bool {
    let balance = backtest(trades);
    balance <= max_loss
}

pub fn main() {
    // Read an input to the t24_zk_program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let trades = sp1_zkvm::io::read::<Vec<Trade>>();
    let max_loss = sp1_zkvm::io::read::<i64>();

    // Compute the n'th fibonacci number using a function from the workspace lib crate.
    let liquidate = prove_liquidation(trades,max_loss);

    sp1_zkvm::io::commit::<bool>(&liquidate);


    // // Encode the public values of the t24_zk_program.
    // let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { n, a, b });
    //
    // // Commit to the public values of the t24_zk_program. The final proof will have a commitment to all the
    // // bytes that were committed to.
    // sp1_zkvm::io::commit_slice(&bytes);
}
