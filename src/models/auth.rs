use crate::config::CONFIG;
use argon2rs::argon2i_simple;

/// Encrypt a password
///
/// Uses the argon2i algorithm.
/// auth_salt is environment configured.
pub fn hash(password: &str) -> String {
    argon2i_simple(&password, &CONFIG.auth_salt)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

/// Verify a password against the encrtypted version
pub fn verify(password: &str, hashed_password: &str) -> bool {
    let hashed = hash(password);
    hashed == hashed_password
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_hashes_a_password() {
        let password = "password";
        let hashed = hash(password);
        assert_ne!(password, hashed);
    }

    #[test]
    fn it_verifies_a_password() {
        let password = "password";
        let hashed = hash(password);
        let verified = verify(password, &hashed);
        assert!(verified);
    }
}
