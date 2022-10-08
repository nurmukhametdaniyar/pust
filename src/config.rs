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
            enable_digit: args.enable_digit,
            enable_lower: args.enable_lower,
            enable_special: args.enable_special,
            enable_upper: args.enable_upper,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_args_match() {
        let enable_digit = true;
        let enable_lower = true;
        let enable_special = true;
        let enable_upper = true;
        let account_name: String = "test".to_string();
        let length = 20;
        let cli_args = CliArgs {
            account_name,
            length,
            enable_digit,
            enable_lower,
            enable_special,
            enable_upper,
        };
        let config = ConfigArgs::new(&cli_args);
        assert_eq!(config.enable_digit, cli_args.enable_digit);
        assert_eq!(config.enable_lower, cli_args.enable_lower);
        assert_eq!(config.enable_special, cli_args.enable_special);
        assert_eq!(config.enable_upper, cli_args.enable_upper);
    }
}
