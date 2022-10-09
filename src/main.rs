use crate::{alphabet::Alphabet, config::ConfigArgs};
use clap::Parser;
use constants::{
    DEFAULT_DISABLE_DIGIT, DEFAULT_DISABLE_LOWER, DEFAULT_DISABLE_SPECIAL, DEFAULT_DISABLE_UPPER,
    DEFAULT_PASS_LEN,
};

mod alphabet;
mod config;
mod constants;
mod rnger;

#[derive(Parser)]
pub struct CliArgs {
    account_name: String,

    #[clap(default_value=DEFAULT_PASS_LEN, long)]
    length: u32,

    #[clap(default_value=DEFAULT_DISABLE_DIGIT, short_alias='d', long)]
    disable_digit: bool,

    #[clap(default_value=DEFAULT_DISABLE_LOWER, short_alias='l', long)]
    disable_lower: bool,

    #[clap(default_value=DEFAULT_DISABLE_SPECIAL, short_alias='s', long)]
    disable_special: bool,

    #[clap(default_value=DEFAULT_DISABLE_UPPER, short_alias='u', long)]
    disable_upper: bool,
}

fn main() {
    sudo::escalate_if_needed().expect("Can't restart with sudo");
    let args = CliArgs::parse();
    let account_name = args.account_name.to_lowercase();
    let salt = format!("salt+{}+salt", account_name);
    let config = ConfigArgs::new(&args);
    let mut alphabet = Alphabet::new(config, salt);
    println!("{}", alphabet.gen_password(args.length));
}
