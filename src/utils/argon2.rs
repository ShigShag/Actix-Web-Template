use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use log::error;

pub fn hash_password(password: &String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &String, hash: &String) -> bool {
    // Convert hash into PasswordHash type
    let parsed_hash = PasswordHash::new(&hash);

    match parsed_hash {
        Ok(password_hash) => {
            // Verify hash
            match Argon2::default().verify_password(password.as_bytes(), &password_hash) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        Err(err) => {
            error!("Error parsing hash: {}", err);
            false
        }
    }
}
