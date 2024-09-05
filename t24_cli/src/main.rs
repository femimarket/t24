use clap::Parser;
use t24_cli::CargoCli;

fn main() {
    let cli = CargoCli::parse();

    match cli {
        CargoCli::Trial(args) => {
            println!("{:?}",args);
        }
    }
}