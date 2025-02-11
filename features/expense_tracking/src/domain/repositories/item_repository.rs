use async_trait::async_trait;

use crate::domain::entities::Item;

#[async_trait]
pub trait ItemRepository {
    async fn create_or_update(&self, category: &Item) -> Result<Option<Item>, ItemRepositoryError>;

    async fn retrieve_all(&self) -> Result<Vec<Item>, ItemRepositoryError>;
}

#[derive(Debug, PartialEq)]
pub enum ItemRepositoryError {}
