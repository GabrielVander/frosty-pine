use std::collections::HashMap;

use async_trait::async_trait;
use uuid_b64::UuidB64;

use expense_tracking::domain::{
    entities::Brand,
    repositories::{BrandRepository, BrandRepositoryError},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BrandRepositoryInMemoryImpl {
    hash_map: HashMap<UuidB64, Brand>,
}

impl BrandRepositoryInMemoryImpl {
    pub fn new(hash_map: HashMap<UuidB64, Brand>) -> Self {
        Self { hash_map }
    }
}

#[async_trait]
impl BrandRepository for BrandRepositoryInMemoryImpl {
    async fn create_or_update(
        &mut self,
        brand: &Brand,
    ) -> Result<Option<Brand>, BrandRepositoryError> {
        Ok(self.hash_map.insert(brand.id, brand.clone()))
    }

    async fn retrieve_all(&self) -> Result<Vec<Brand>, BrandRepositoryError> {
        Ok(self.hash_map.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use uuid_b64::UuidB64;

    use super::BrandRepositoryInMemoryImpl;
    use expense_tracking::domain::{
        entities::Brand,
        repositories::{BrandRepository, BrandRepositoryError},
    };

    fn given_empty_repository() -> BrandRepositoryInMemoryImpl {
        given_repository_with(Vec::new())
    }

    fn given_repository_with(brands: Vec<Brand>) -> BrandRepositoryInMemoryImpl {
        BrandRepositoryInMemoryImpl::new(
            brands
                .into_iter()
                .map(|b| (b.id, b))
                .collect::<HashMap<UuidB64, Brand>>(),
        )
    }

    fn given_new_brand() -> Brand {
        Brand::new(None, String::default())
    }

    macro_rules! retrieve_all {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[tokio::test]
            async fn $name() {
                let brands: &Vec<Brand> = $value;

                let repository: BrandRepositoryInMemoryImpl = given_repository_with(brands.clone());

                let result: Result<Vec<Brand>, BrandRepositoryError> =
                    repository.retrieve_all().await;

                assert!(result.is_ok(), "Expected Ok, got {:?}", result);

                let mut expected_brands: Vec<Brand> = brands.clone();
                expected_brands.sort_by(|a, b| a.id.cmp(&b.id));

                let mut sorted_result: Vec<Brand> = result.unwrap();
                sorted_result.sort_by(|a, b| a.id.cmp(&b.id));

                assert_eq!(
                    sorted_result, expected_brands,
                    "Expected {:?}, but got {:?}",
                    expected_brands, sorted_result
                )
            }
        )*
        }
    }

    retrieve_all! {
        no_brands: &vec![],
        one_brand: &vec![given_new_brand()],
        multiple_brands: &vec![given_new_brand(), given_new_brand(), given_new_brand()],
    }

    #[tokio::test]
    async fn create_or_update_should_add_new_brand_given_empty_repository() {
        let brand: Brand = Brand::new(None, "New Brand".into());
        let mut repository: BrandRepositoryInMemoryImpl = given_empty_repository();

        let result: Result<Option<Brand>, BrandRepositoryError> =
            repository.create_or_update(&brand).await;
        let expected: Result<Option<Brand>, BrandRepositoryError> = Ok(None);

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }

    #[tokio::test]
    async fn create_or_update_should_add_new_brand_given_full_repository() {
        let brand: Brand = Brand::new(None, "New Brand".into());
        let mut repository: BrandRepositoryInMemoryImpl = given_repository_with(vec![
            given_new_brand(),
            given_new_brand(),
            given_new_brand(),
        ]);

        let result: Result<Option<Brand>, BrandRepositoryError> =
            repository.create_or_update(&brand).await;
        let expected: Result<Option<Brand>, BrandRepositoryError> = Ok(None);

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }

    #[tokio::test]
    async fn create_or_update_should_update_brand_given_full_repository() {
        let old_brand: Brand = Brand::new(None, "New Brand".into());
        let updated_brand: Brand = Brand::new(Some(old_brand.id), "New Updated Brand".into());

        let mut repository: BrandRepositoryInMemoryImpl = given_repository_with(vec![
            given_new_brand(),
            old_brand.clone(),
            given_new_brand(),
        ]);

        let result: Result<Option<Brand>, BrandRepositoryError> =
            repository.create_or_update(&updated_brand).await;
        let expected: Result<Option<Brand>, BrandRepositoryError> = Ok(Some(old_brand));

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }
}
