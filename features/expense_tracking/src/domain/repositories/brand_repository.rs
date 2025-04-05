use async_trait::async_trait;

use crate::domain::entities::Brand;

#[async_trait]
pub trait BrandRepository: std::fmt::Debug {
    async fn create(&mut self, brand: &Brand) -> Result<Brand, BrandRepositoryError>;

    async fn retrieve_all(&self) -> Result<Vec<Brand>, BrandRepositoryError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum BrandRepositoryError {
    UnableToSaveBrand(String),
    BrandAlreadyExists,
}
