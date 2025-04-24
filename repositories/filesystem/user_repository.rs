use serde_json;
use tokio::fs;
use std::path::PathBuf;

pub struct FileSystemUserRepository {
    file_path: PathBuf,
}

impl FileSystemUserRepository {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }
}

#[async_trait]
impl Repository<User, String> for FileSystemUserRepository {
    async fn save(&mut self, user: User) -> Result<(), Box<dyn Error>> {
        let mut users = self.load_all().await?;
        users.insert(user.user_id.clone(), user);
        let json = serde_json::to_string(&users)?;
        fs::write(&self.file_path, json).await?;
        Ok(())
    }
    // ... other methods
}
