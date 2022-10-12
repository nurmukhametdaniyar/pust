use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{
    env::var,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub struct Hashing;

impl Hashing {
    pub fn hash(s: String) -> Result<String, String> {
        let from_env = format!("{}/.pust/salt", var("HOME").unwrap());
        let path = Path::new(&from_env);
        if !path.exists() {
            match File::create(path) {
                Ok(mut file) => {
                    let rand_string: String = thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(30)
                        .map(char::from)
                        .collect();
                    if let Err(_) = file.write_all(rand_string.as_bytes()) {
                        return Err(String::from("Error creating hex file"));
                    };
                }
                Err(_) => return Err(String::from("Error creating file")),
            }
        }
        let hash = std::fs::read_to_string(path).expect("Unable to read file");
        let joined_string = format!("{}+{}", s, hash);
        let digest = md5::compute(joined_string.as_bytes());
        Ok(format!("{:x}", digest))
    }

    pub fn get_salt() -> String {
        let from_env = format!("{}/.pust/salt", var("HOME").unwrap());
        let path = Path::new(&from_env);
        std::fs::read_to_string(path).expect("Unable to read file")
    }

    pub fn delete_salt_file() -> Result<(), String> {
        if let Err(_) = fs::remove_file(format!("{}/.pust/salt", var("HOME").unwrap())) {
            return Err(String::from("Error deleting salt"));
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    // TODO: Add tests
}
