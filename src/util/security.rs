use anyhow::{Ok, Result, anyhow};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use secrecy::{ExposeSecret, SecretString};

pub fn hash_password(plain: &SecretString) -> Result<String, anyhow::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(plain.expose_secret().as_bytes(), &salt)
        .map_err(|e| anyhow!("Error has {}", e))?;
    let result = anyhow::Ok(password_hash.to_string());
    result
}

pub fn verify_passwrod(secret_string: &SecretString, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash).map_err(|err| anyhow!("{}", err))?;
    let argon = Argon2::default();
    let result = Ok(argon
        .verify_password(secret_string.expose_secret().as_bytes(), &parsed_hash)
        .is_ok());
    result
}
