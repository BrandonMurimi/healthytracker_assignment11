use crate::repositories::{
    UserRepository, 
    inmemory::InMemoryUserRepository
};

pub enum StorageType {
    Memory,
    // FileSystem,
    // Database,
}

pub struct RepositoryFactory;

impl RepositoryFactory {
    pub fn user_repository(storage_type: StorageType) -> Box<dyn UserRepository> {
        match storage_type {
            StorageType::Memory => Box::new(InMemoryUserRepository::new()),
        }
          }
}
