use bcrypt::{hash, verify};

pub fn generate_password(password: &str) -> Result<String, String> {
    const COST: u32 = 13;

    match hash(password, COST) {
        Ok(hashed_password) => Ok(hashed_password),
        Err(e) => Err(format!("Failed to hash password: {}", e))
    }
}

pub fn check_password(password: &str, hash: &str) -> Result<bool, String> {

    match verify(password, hash) {
        Ok(is_password) => Ok(is_password),
        Err(e) => Err(format!("Failed to verify password: {}", e))
    }
}
