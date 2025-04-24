#[tokio::test]
async fn test_crud_operations() {
    let mut repo = InMemoryUserRepository::new();
    let user = User::new("alice@fit.com".into());
    
    // Create
    repo.save(user.clone()).await.unwrap();
    
    // Read
    let found = repo.find_by_id(user.user_id).await.unwrap();
    assert!(found.is_some());
    
    // Delete
    repo.delete(user.user_id).await.unwrap();
    assert!(repo.find_by_id(user.user_id).await.unwrap().is_none());
}
