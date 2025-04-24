//! Defines generic and entity-specific repository traits

use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Repository<T, ID> {
    async fn save(&mut self, entity: T) -> Result<(), Box<dyn Error>>;
    async fn find_by_id(&self, id: ID) -> Result<Option<T>, Box<dyn Error>>;
    async fn find_all(&self) -> Result<Vec<T>, Box<dyn Error>>;
    async fn delete(&mut self, id: ID) -> Result<(), Box<dyn Error>>;
}

// Entity-specific marker traits
pub trait UserRepository: Repository<User, String> + Send + Sync {}
pub trait ActivityRepository: Repository<Activity, String> + Send + Sync {}
