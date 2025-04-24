use std::collections::HashMap;
use tokio::sync::RwLock;
use super::UserRepository;

pub struct InMemoryUserRepository {
    storage: RwLock<HashMap<String, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Repository<User, String> for InMemoryUserRepository {
    async fn save(&mut self, user: User) -> Result<(), Box<dyn Error>> {
        self.storage.write().await.insert(user.user_id.clone(), user);
        Ok(())
    }
    // ... other CRUD methods
}

impl UserRepository for InMemoryUserRepository {}

