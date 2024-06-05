use argon2;
use r2d2_redis::redis::RedisError;
use serde_json::Error as SerdeError;
use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    R2D2Error(String),
    DieselError(diesel::result::Error),
    RedisOperationError(RedisError),
    SerializationError(SerdeError),
    DeserializationError(SerdeError),
    Argon2Error(argon2::password_hash::Error),
    UserAlreadyExists(String),
}

impl From<diesel::result::Error> for DatabaseError {
    fn from(err: diesel::result::Error) -> Self {
        DatabaseError::DieselError(err)
    }
}

impl From<r2d2::Error> for DatabaseError {
    fn from(err: r2d2::Error) -> Self {
        DatabaseError::R2D2Error(err.to_string())
    }
}

impl From<SerdeError> for DatabaseError {
    fn from(err: SerdeError) -> Self {
        if err.is_data() {
            DatabaseError::DeserializationError(err)
        } else {
            DatabaseError::SerializationError(err)
        }
    }
}

impl From<RedisError> for DatabaseError {
    fn from(err: RedisError) -> Self {
        DatabaseError::RedisOperationError(err)
    }
}

impl From<argon2::password_hash::Error> for DatabaseError {
    fn from(err: argon2::password_hash::Error) -> Self {
        DatabaseError::Argon2Error(err)
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::R2D2Error(msg) => {
                write!(f, "Connection Pool Error: {}", msg)
            }
            DatabaseError::DieselError(err) => {
                write!(f, "Database Operation Error: {}", err)
            }
            DatabaseError::RedisOperationError(ref err) => {
                write!(f, "Redis operation error: {}", err)
            }
            DatabaseError::SerializationError(ref err) => {
                write!(f, "Serialization error: {}", err)
            }
            DatabaseError::DeserializationError(ref err) => {
                write!(f, "Deserialization error: {}", err)
            }
            DatabaseError::Argon2Error(ref err) => {
                write!(f, "Argon2 error: {}", err)
            }
            DatabaseError::UserAlreadyExists(msg) => {
                write!(f, "User already exists: {}", msg)
            }
        }
    }
}
