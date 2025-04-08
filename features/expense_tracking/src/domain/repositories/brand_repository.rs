use async_trait::async_trait;

use crate::domain::entities::Brand;

#[async_trait]
pub trait BrandRepository: std::fmt::Debug {
    async fn create(&self, brand: &Brand) -> Result<Brand, BrandRepositoryCreateError>;

    async fn retrieve_all(&self) -> Result<Vec<Brand>, BrandRepositoryRetrieveAllError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum BrandRepositoryCreateError {
    UnableToSaveBrand(String),
    BrandAlreadyExists,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BrandRepositoryRetrieveAllError {
    UnableToRetrieveBrands(String),
}
