use bcrypt::hash;

pub fn generate_password(password: &str) -> Result<String, String> {
    const COST: u32 = 5;

    match hash(password, COST) {
        Ok(hashed_password) => Ok(hashed_password),
        Err(e) => Err(format!("Failed to hash password: {}", e))
    }
}

pub fn check_password(password: &str) -> Result<String, String> {
    todo()
}
