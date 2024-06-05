use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use r2d2_redis::redis::Commands;
use r2d2_redis::{r2d2 as garnet_r2d2, RedisConnectionManager};
use serde_json;

use super::errors::DatabaseError;
use crate::models::users::{NewUser, User};
use crate::schema::users::dsl as user_dsl;

// Type alias for using the specific Postgres connection pool
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Cache = garnet_r2d2::Pool<RedisConnectionManager>;

#[derive(Clone)]
pub struct Database {
    pub db_pool: Pool,
    pub cache_pool: Cache,
}

impl Database {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load the DATABASE_URL from .env or environment
        let database_url = std::env::var("DATABASE_URL")?;
        let garnet_url = std::env::var("GARNET_URL")?;

        // Set up database connection pool
        let db_manager = ConnectionManager::<PgConnection>::new(database_url);
        let db_pool = r2d2::Pool::builder()
            .build(db_manager)
            .expect("Failed to create pool.");

        let cache_manager = RedisConnectionManager::new(garnet_url)?;
        let cache_pool = r2d2::Pool::builder().build(cache_manager)?;

        // Setup up cache connection pool

        Ok(Database {
            db_pool,
            cache_pool,
        })
    }

    // Users
    // Inserts a new user into the database
    pub fn create_user(&self, new_user: &NewUser) -> Result<(), DatabaseError> {
        let mut db_conn = self.db_pool.get()?;
        let mut cache_conn = self.cache_pool.get()?;

        // Insert user into database
        let user: User = diesel::insert_into(user_dsl::users)
            .values(new_user)
            .get_result(&mut db_conn)?;

        // Serialize object and insert it into cache
        let user_serialized = serde_json::to_string(&user)?;

        // Insert the challenge with TTL into Redis | Format entry to avoid collisions with other db tables
        cache_conn.set(format!("user:{}", user.id), user_serialized)?;

        Ok(())
    }

    // Retrieves a user by their email
    pub fn get_user_by_email(&self, user_email: &str) -> Result<User, DatabaseError> {
        let mut cache_conn = self.cache_pool.get()?;
        let cache_key = format!("user_email:{}", user_email);

        // Try to retrieve the user from the cache
        if let Some(user_serialized) = cache_conn.get::<_, Option<String>>(&cache_key)? {
            // Deserialize and return if found in cache
            let user: User = serde_json::from_str(&user_serialized)?;
            return Ok(user);
        }

        // If not found in the cache, fetch from the database
        let mut db_conn = self.db_pool.get()?;
        let user = user_dsl::users
            .filter(user_dsl::email.eq(user_email))
            .first(&mut db_conn)?;

        // Serialize and cache the user
        let user_serialized = serde_json::to_string(&user)?;
        cache_conn.set_ex(cache_key, user_serialized, 3600)?;

        Ok(user)
    }
}
