use clap::Parser;


#[derive(Parser)] // requires `derive` feature
pub enum CargoCli {
    Trial(TrialArgs),
}

/// Start trading trial
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct TrialArgs {
    /// Name of the person to greet
    #[arg(
        short = 'a',
        long,
        default_value = "10k",
        value_parser = clap::builder::PossibleValuesParser::new(["10k", "100k"]),
    )]
    account: String,

}