use async_trait::async_trait;

use crate::domain::entities::Product;

#[async_trait]
pub trait ProductRepository {
    async fn create_or_update(
        &self,
        category: &Product,
    ) -> Result<Option<Product>, ProductRepositoryError>;

    async fn retrieve_all(&self) -> Result<Vec<Product>, ProductRepositoryError>;
}

#[derive(Debug, PartialEq)]
pub enum ProductRepositoryError {}
