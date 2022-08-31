//! Handle JWTs, hash passwords, and Identity Service

use crate::config::CONFIG;
use argon2rs::argon2i_simple;

/// Encrypt a password
///
/// Uses the argon2i algorithm.
/// auth_salt is environment-configured.
pub fn hash(password: &str) -> String {
    argon2i_simple(&password, &CONFIG.auth_salt)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    static EMAIL: &str = "test@test.com";

    #[test]
    fn it_hashes_a_password() {
        let password = "password";
        let hashed = hash(password);
        assert_ne!(password, hashed);
    }

    #[test]
    fn it_matches_2_hashed_passwords() {
        let password = "password";
        let hashed = hash(password);
        let hashed_again = hash(password);
        assert_eq!(hashed, hashed_again);
    }
}
