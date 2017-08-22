use ring::{digest, pbkdf2};
pub use super::super::error;
pub use auth::default;

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;

pub type Credential = [u8; CREDENTIAL_LEN];

pub struct GoofyCrypto {
    pbkdf2_iterations: u32,
    db_salt_component: [u8; 16],
}

impl GoofyCrypto {
    pub fn new() -> GoofyCrypto {
        GoofyCrypto {
            pbkdf2_iterations: 100_000,
            db_salt_component: [
                // This value was generated from a secure PRNG.
                0xd6,
                0x26,
                0x98,
                0xda,
                0xf4,
                0xdc,
                0x50,
                0x52,
                0x24,
                0xf2,
                0x27,
                0xd1,
                0xfe,
                0x39,
                0x01,
                0x8a,
            ],
        }
    }

    //The username is actually an email
    pub fn encrypt_password(&mut self, username: &str, password: &str) -> error::Result<String> {

        let salt = self.salt(username);
        let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            DIGEST_ALG,
            self.pbkdf2_iterations,
            &salt,
            password.as_bytes(),
            &mut to_store,
        );

        let data = String::from_utf8(to_store.to_vec()).map_err(|_| {
            error::Error::CryptoError("Error parsing password signature".to_string())
        })?;
        Ok(data)
    }

    pub fn verify_password(&self, username: &str, actual_password: &str, attempted_password: &str) -> error::Result<()> {
        let salt = self.salt(username);

        //The error is not returned here.
        pbkdf2::verify(
            DIGEST_ALG,
            self.pbkdf2_iterations,
            &salt,
            attempted_password.as_bytes(),
            actual_password.as_bytes(),
        ).map_err(|_| {
            error::Error::CryptoError("Error verifying password signature".to_string())
        })?;

        Ok(())
    }

    // The salt should have a user-specific component so that an attacker
    // cannot crack one password for multiple users in the database. It
    // should have a database-unique component so that an attacker cannot
    // crack the same user's password across databases in the unfortunate
    // but common case that the user has used the same password for
    // multiple systems.
    fn salt(&self, username: &str) -> Vec<u8> {
        let mut salt = Vec::with_capacity(self.db_salt_component.len() + username.as_bytes().len());
        salt.extend(self.db_salt_component.as_ref());
        salt.extend(username.as_bytes());
        salt
    }
}

/*
fn main() {
    // Normally these parameters would be loaded from a configuration file.
    let mut db = GoofyCrypto {
        pbkdf2_iterations: 100_000,
        db_salt_component: [
            // This value was generated from a secure PRNG.
            0xd6,
            0x26,
            0x98,
            0xda,
            0xf4,
            0xdc,
            0x50,
            0x52,
            0x24,
            0xf2,
            0x27,
            0xd1,
            0xfe,
            0x39,
            0x01,
            0x8a,
        ],
        storage: HashMap::new(),
    };

    db.store_password("alice", "@74d7]404j|W}6u");

    // An attempt to log in with the wrong password fails.
    assert!(db.verify_password("alice", "wrong password").is_err());

    // Normally there should be an expoentially-increasing delay between
    // attempts to further protect against online attacks.

    // An attempt to log in with the right password succeeds.
    assert!(db.verify_password("alice", "@74d7]404j|W}6u").is_ok());
}
*/
