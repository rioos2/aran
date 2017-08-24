use crypto::pbkdf2::{pbkdf2_simple, pbkdf2_check};

use super::super::error;
pub use auth::default;

pub struct GoofyCrypto {
    pbkdf2_iterations: u32,
}

impl GoofyCrypto {
    pub fn new() -> GoofyCrypto {
        GoofyCrypto { pbkdf2_iterations: 1024 }
    }

    //The username is actually an email
    pub fn encrypt_password(&mut self, password: &str) -> error::Result<String> {
        match pbkdf2_simple(password, self.pbkdf2_iterations) {
            Ok(result) => return Ok(result),
            Err(e) => return Err(error::Error::IO(e)),
        }
    }

    pub fn verify_password(&mut self, actual_password: &str, attempted_password: &str) -> error::Result<()> {
        let verified = pbkdf2_check(attempted_password, &actual_password[..]);

        match verified {
            Ok(result) => {
                if !result {
                    return Err(error::Error::Auth(default::AuthErr {
                        error: String::from("Password match not found"),
                        error_description: format!("Verifyer returned {}", result),
                    }));
                }
            }
            Err(e) => {
                return Err(error::Error::Auth(default::AuthErr {
                    error: String::from("Password failed to check"),
                    error_description: format!("{}", e),
                }));
            }
        }

        Ok(())

    }
}
