//! An end-to-end example of using the SP1 SDK to generate a proof of a t24_zk_program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use alloy_sol_types::SolType;
use clap::Parser;
// use fibonacci_lib::PublicValuesStruct;
use sp1_sdk::{ProverClient, SP1Stdin};
use t24_lib::tick::Tick;
use t24_lib::trade::Trade;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const T24_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long)]
    trades: String,

    #[clap(long,allow_negative_numbers(true))]
    max_loss: i64,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    let trades = serde_json::from_str::<Vec<Trade>>(&args.trades).unwrap();
    let max_loss = args.max_loss;;

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&trades);
    stdin.write(&max_loss);

    println!("n: {:?}", args.trades);

    if args.execute {
        // Execute the t24_zk_program
        let (output, report) = client.execute(T24_ELF, stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        // let decoded = PublicValuesStruct::abi_decode(output.as_slice(), true).unwrap();
        // let PublicValuesStruct { n, a, b } = decoded;
        println!("n: {:?}", output.as_slice());
        // println!("a: {}", a);
        // println!("b: {}", b);
        //
        // let (expected_a, expected_b) = fibonacci_lib::fibonacci(n);
        // assert_eq!(a, expected_a);
        // assert_eq!(b, expected_b);
        // println!("Values are correct!");
        //
        // // Record the number of cycles executed.
        // println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the t24_zk_program for proving.
        let (pk, vk) = client.setup(T24_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
