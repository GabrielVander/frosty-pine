use async_trait::async_trait;

use crate::domain::entities::Store;

#[async_trait]
pub trait StoreRepository {
    async fn create_or_update(
        &self,
        category: &Store,
    ) -> Result<Option<Store>, StoreRepositoryError>;

    async fn retrieve_all(&self) -> Result<Vec<Store>, StoreRepositoryError>;
}

#[derive(Debug, PartialEq)]
pub enum StoreRepositoryError {}
