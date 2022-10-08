use crate::rnger::Rnger;
use crate::{config::ConfigArgs, constants::SPECIAL_CHARS};

pub struct Alphabet {
    chars: Vec<char>,
    rng: Rnger,
}

impl Alphabet {
    pub fn new(config: ConfigArgs, salt: String) -> Self {
        let mut chars: Vec<char> = Vec::with_capacity(76);
        Self::append_special(config.enable_special, &mut chars);
        Self::append_upper(config.enable_upper, &mut chars);
        Self::append_lower(config.enable_lower, &mut chars);
        Self::append_digits(config.enable_digit, &mut chars);
        let rng = Rnger::new(salt);
        Alphabet {
            chars: chars,
            rng: rng,
        }
    }

    pub fn gen_password(&mut self, length: u32) -> String {
        if self.chars.len() == 0 {
            return String::new();
        }
        let mut password: Vec<char> = Vec::with_capacity(length as usize);
        for _ in 0..length {
            password.push(self.get_char());
        }
        password.iter().collect()
    }

    fn get_char(&mut self) -> char {
        let index = self.rng.gen::<usize>() % self.chars.len();
        self.chars[index]
    }

    fn append_special(enable_special: bool, vec: &mut Vec<char>) {
        if enable_special {
            vec.append(&mut SPECIAL_CHARS.to_vec());
        }
    }

    fn append_upper(enable_upper: bool, vec: &mut Vec<char>) {
        if enable_upper {
            for i in 'A' as u8..'Z' as u8 + 1 {
                vec.push(i as char);
            }
        }
    }

    fn append_lower(enable_lower: bool, vec: &mut Vec<char>) {
        if enable_lower {
            for i in 'a' as u8..'z' as u8 + 1 {
                vec.push(i as char);
            }
        }
    }

    fn append_digits(enable_digits: bool, vec: &mut Vec<char>) {
        if enable_digits {
            for i in '0' as u8..='9' as u8 + 1 {
                vec.push(i as char);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use super::*;

    #[test]
    fn should_create_new_alphabet() {
        let mut rng = rand::thread_rng();
        let enable_digits = rng.gen::<bool>();
        let enable_lower = rng.gen::<bool>();
        let enable_special = rng.gen::<bool>();
        let enable_upper = rng.gen::<bool>();
        let config = ConfigArgs {
            enable_digit: enable_digits,
            enable_lower: enable_lower,
            enable_special: enable_special,
            enable_upper: enable_upper,
        };
        let salt = String::from("test");
        let alphabet = Alphabet::new(config, salt);
        let mut vec: Vec<char> = Vec::with_capacity(76);
        Alphabet::append_special(enable_special, &mut vec);
        Alphabet::append_upper(enable_upper, &mut vec);
        Alphabet::append_lower(enable_lower, &mut vec);
        Alphabet::append_digits(enable_digits, &mut vec);
        assert_eq!(alphabet.chars, vec);
    }

    #[test]
    fn should_return_empty_password() {
        let length: u32 = 16;
        let config = ConfigArgs {
            enable_digit: false,
            enable_lower: false,
            enable_special: false,
            enable_upper: false,
        };
        let salt = String::from("test");
        let mut alphabet = Alphabet::new(config.clone(), salt);
        assert_eq!(alphabet.gen_password(length), String::new(),);
    }

    #[test]
    fn should_generate_identical_passwords() {
        let length: u32 = 16;
        let mut rng = rand::thread_rng();
        let enable_digits = rng.gen::<bool>();
        let enable_lower = rng.gen::<bool>();
        let enable_special = rng.gen::<bool>();
        let enable_upper = rng.gen::<bool>();
        let config = ConfigArgs {
            enable_digit: enable_digits,
            enable_lower: enable_lower,
            enable_special: enable_special,
            enable_upper: enable_upper,
        };
        let salt_1 = String::from("test");
        let salt_2 = String::from("test");
        let mut alphabet_1 = Alphabet::new(config.clone(), salt_1);
        let mut alphabet_2 = Alphabet::new(config, salt_2);
        assert_eq!(
            alphabet_1.gen_password(length),
            alphabet_2.gen_password(length)
        );
    }

    #[test]
    fn should_not_generate_identical_passwords() {
        let length: u32 = 16;
        let mut rng = rand::thread_rng();
        let enable_digits = true;
        let enable_lower = rng.gen::<bool>();
        let enable_special = rng.gen::<bool>();
        let enable_upper = rng.gen::<bool>();
        let config = ConfigArgs {
            enable_digit: enable_digits,
            enable_lower: enable_lower,
            enable_special: enable_special,
            enable_upper: enable_upper,
        };
        let salt_1 = String::from("test");
        let salt_2 = String::from("test_1");
        let mut alphabet_1 = Alphabet::new(config.clone(), salt_1);
        let mut alphabet_2 = Alphabet::new(config, salt_2);
        assert_ne!(
            alphabet_1.gen_password(length),
            alphabet_2.gen_password(length)
        );
    }

    #[test]
    fn should_generate_identical_chars() {
        let mut rng = rand::thread_rng();
        let enable_digits = true;
        let enable_lower = rng.gen::<bool>();
        let enable_special = rng.gen::<bool>();
        let enable_upper = rng.gen::<bool>();
        let config = ConfigArgs {
            enable_digit: enable_digits,
            enable_lower: enable_lower,
            enable_special: enable_special,
            enable_upper: enable_upper,
        };
        let salt_1 = String::from("test");
        let salt_2 = String::from("test");
        let mut alphabet_1 = Alphabet::new(config.clone(), salt_1);
        let mut alphabet_2 = Alphabet::new(config, salt_2);
        for _ in 0..10 {
            assert_eq!(alphabet_1.get_char(), alphabet_2.get_char());
        }
    }

    #[test]
    fn should_append_special() {
        let mut vec: Vec<char> = Vec::new();
        Alphabet::append_special(true, &mut vec);
        assert_eq!(vec, SPECIAL_CHARS);
    }

    #[test]
    fn should_not_append_special() {
        let mut vec: Vec<char> = Vec::new();
        Alphabet::append_special(false, &mut vec);
        assert_ne!(vec, SPECIAL_CHARS);
        assert!(vec.len() == 0);
    }

    #[test]
    fn should_append_upper() {
        let mut vec: Vec<char> = Vec::with_capacity(26);
        let mut upper_letters: Vec<char> = Vec::with_capacity(26);
        for i in 'A' as u8..'Z' as u8 + 1 {
            upper_letters.push(i as char);
        }
        Alphabet::append_upper(true, &mut vec);
        assert_eq!(vec, upper_letters);
    }

    #[test]
    fn should_not_append_upper() {
        let mut vec: Vec<char> = Vec::new();
        Alphabet::append_upper(false, &mut vec);
        assert!(vec.len() == 0);
    }

    #[test]
    fn should_append_lower() {
        let mut vec: Vec<char> = Vec::with_capacity(26);
        let mut lower_letters: Vec<char> = Vec::with_capacity(26);
        for i in 'a' as u8..'z' as u8 + 1 {
            lower_letters.push(i as char);
        }
        Alphabet::append_lower(true, &mut vec);
        assert_eq!(vec, lower_letters);
    }

    #[test]
    fn should_not_append_lower() {
        let mut vec: Vec<char> = Vec::new();
        Alphabet::append_lower(false, &mut vec);
        println!("Vec len: {}", vec.len());
        assert!(vec.len() == 0);
    }

    #[test]
    fn should_append_digits() {
        let mut vec: Vec<char> = Vec::with_capacity(10);
        let mut digits: Vec<char> = Vec::with_capacity(10);
        for i in '0' as u8..='9' as u8 + 1 {
            digits.push(i as char);
        }
        Alphabet::append_digits(true, &mut vec);
        assert_eq!(vec, digits);
    }

    #[test]
    fn should_not_append_digits() {
        let mut vec: Vec<char> = Vec::new();
        Alphabet::append_digits(false, &mut vec);
        assert!(vec.len() == 0);
    }
}
