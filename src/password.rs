use std::{
    env::var,
    fs::{self, File},
    io::Write,
    path::Path,
};

use rpassword::read_password;

use crate::hashing::Hashing;

pub struct Password;

impl Password {
    fn check_password_exists() -> bool {
        let from_env = format!("{}/.pust/auth", var("HOME").unwrap());
        let path = Path::new(&from_env);
        path.exists()
    }

    fn check_password(input_password: &String) -> Result<bool, String> {
        let from_env = format!("{}/.pust/auth", var("HOME").unwrap());
        let password_path = Path::new(&from_env);
        let hashed_input_result = Hashing::hash(input_password);
        match hashed_input_result {
            Ok(hashed_password) => {
                let original_hashed_password =
                    std::fs::read_to_string(password_path).expect("Unable to read password file");
                if hashed_password == original_hashed_password {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
            Err(err) => return Err(err),
        }
    }

    pub fn read_password() -> Result<String, String> {
        let password_exists = Self::check_password_exists();
        if password_exists {
            let mut incorrect_count: u32 = 0;
            loop {
                print!("Enter master password: ");
                std::io::stdout().flush().unwrap();
                let password = read_password().unwrap();
                let check_password_result = Self::check_password(&password);
                match check_password_result {
                    Ok(result) => {
                        if result {
                            return Ok(password);
                        } else {
                            incorrect_count += 1;
                            if incorrect_count >= 3 {
                                let _ = Self::delete_password_file();
                                let _ = Hashing::delete_salt_file();
                                println!("Deleted password file");
                                return Err(String::from("Incorrect password"));
                            }
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        return Err(err);
                    }
                }
            }
        } else {
            print!("Create master password: ");
            std::io::stdout().flush().unwrap();
            let password = read_password().unwrap();
            print!("Re-enter master password: ");
            std::io::stdout().flush().unwrap();
            let password_repeat = read_password().unwrap();
            if password == password_repeat {
                let create_password_file_result = Self::create_password_file();
                match create_password_file_result {
                    Ok(mut file) => {
                        let hashed_password_result = Hashing::hash(&password);
                        match hashed_password_result {
                            Ok(hashed_password) => {
                                file.write_all(hashed_password.as_bytes())
                                    .expect("Error saving password");
                                return Ok(password);
                            }
                            Err(err) => return Err(err),
                        }
                    }
                    Err(err) => return Err(err),
                }
            } else {
                return Err(String::from("Passwords don't match"));
            }
        }
    }

    fn create_password_file() -> Result<File, String> {
        let from_env = format!("{}/.pust/auth", var("HOME").unwrap());
        let path = Path::new(&from_env);
        let file_result = File::create(path);
        match file_result {
            Ok(file) => {
                return Ok(file);
            }
            Err(_) => return Err(String::from("Error creating file")),
        }
    }

    fn delete_password_file() -> Result<(), String> {
        if let Err(_) = fs::remove_file(format!("{}/.pust/auth", var("HOME").unwrap())) {
            return Err(String::from("Error deleting password"));
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    // TODO: Add tests
}
