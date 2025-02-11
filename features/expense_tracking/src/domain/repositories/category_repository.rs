use async_trait::async_trait;

use crate::domain::entities::Category;

#[async_trait]
pub trait CategoryRepository {
    async fn create_or_update(
        &mut self,
        category: &Category,
    ) -> Result<Option<Category>, CategoryRepositoryError>;

    async fn retrieve_all(&self) -> Result<Vec<Category>, CategoryRepositoryError>;
}

#[derive(Debug, PartialEq)]
pub enum CategoryRepositoryError {}
