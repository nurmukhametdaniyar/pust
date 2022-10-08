use crate::{alphabet::Alphabet, config::ConfigArgs};
use clap::Parser;
use constants::{
    DEFAULT_ENABLE_DIGIT, DEFAULT_ENABLE_LOWER, DEFAULT_ENABLE_SPECIAL, DEFAULT_ENABLE_UPPER,
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

    #[clap(default_value=DEFAULT_ENABLE_DIGIT, short_alias='d', long)]
    enable_digit: bool,

    #[clap(default_value=DEFAULT_ENABLE_LOWER, short_alias='l', long)]
    enable_lower: bool,

    #[clap(default_value=DEFAULT_ENABLE_SPECIAL, short_alias='s', long)]
    enable_special: bool,

    #[clap(default_value=DEFAULT_ENABLE_UPPER, short_alias='u', long)]
    enable_upper: bool,
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
