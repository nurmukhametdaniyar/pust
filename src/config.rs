use crate::CliArgs;

#[derive(Clone, Copy)]
pub struct ConfigArgs {
    pub enable_digit: bool,
    pub enable_lower: bool,
    pub enable_special: bool,
    pub enable_upper: bool,
}

impl ConfigArgs {
    pub fn new(args: &CliArgs) -> Self {
        ConfigArgs {
            enable_digit: !args.disable_digit,
            enable_lower: !args.disable_lower,
            enable_special: !args.disable_special,
            enable_upper: !args.disable_upper,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_args_match() {
        let disable_digit = false;
        let disable_lower = false;
        let disable_special = false;
        let disable_upper = false;
        let account_name: String = "test".to_string();
        let length = 20;
        let cli_args = CliArgs {
            account_name,
            length,
            disable_digit,
            disable_lower,
            disable_special,
            disable_upper,
        };
        let config = ConfigArgs::new(&cli_args);
        assert_eq!(config.enable_digit, !cli_args.disable_digit);
        assert_eq!(config.enable_lower, !cli_args.disable_lower);
        assert_eq!(config.enable_special, !cli_args.disable_special);
        assert_eq!(config.enable_upper, !cli_args.disable_upper);
    }
}
