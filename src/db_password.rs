use bcrypt::{hash, verify, DEFAULT_COST};

pub fn get_salt_hash_password(password: &String) -> anyhow::Result<String> {
    let data = hash(&password, DEFAULT_COST)?;
    anyhow::Ok(data)
}

pub fn verify_salt_hash_password(password: &String, hash: &String) -> anyhow::Result<bool> {
    let valid = verify(password, hash)?;
    anyhow::Ok(valid)
}