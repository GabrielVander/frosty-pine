use async_trait::async_trait;

use crate::domain::entities::Brand;

#[async_trait]
pub trait BrandRepository: std::fmt::Debug {
    /// If the brand was created, [`None`] is returned.
    ///
    /// If the brand was updated the old
    /// value is returned
    async fn create_or_update(
        &mut self,
        brand: &Brand,
    ) -> Result<Option<Brand>, BrandRepositoryError>;

    async fn retrieve_all(&self) -> Result<Vec<Brand>, BrandRepositoryError>;
}

#[derive(Debug, PartialEq)]
pub enum BrandRepositoryError {}
