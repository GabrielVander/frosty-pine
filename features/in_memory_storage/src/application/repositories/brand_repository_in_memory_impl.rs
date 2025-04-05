use std::collections::HashMap;

use async_trait::async_trait;
use uuid_b64::UuidB64;

use expense_tracking::domain::{
    entities::Brand,
    repositories::{BrandRepository, BrandRepositoryError},
};

#[derive(Debug, Clone, PartialEq)]
pub struct BrandRepositoryInMemoryImpl {
    hash_map: HashMap<String, Brand>,
}

impl BrandRepositoryInMemoryImpl {
    pub fn new(hash_map: HashMap<String, Brand>) -> Self {
        Self { hash_map }
    }
}

#[async_trait]
impl BrandRepository for BrandRepositoryInMemoryImpl {
    async fn create(&self, brand: &Brand) -> Result<Brand, BrandRepositoryError> {
        if (self.hash_map.contains_key(&brand.name)) {
            return Err(BrandRepositoryError::BrandAlreadyExists);
        }

        self.hash_map.insert(brand.name.clone(), brand.clone());
        Ok(brand.clone())
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
                .map(|b| (b.name.clone(), b.clone()))
                .collect::<HashMap<String, Brand>>(),
        )
    }

    fn given_new_brand() -> Brand {
        Brand::new(String::default())
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
                expected_brands.sort_by(|a, b| a.name.cmp(&b.name));

                let mut sorted_result: Vec<Brand> = result.unwrap();
                sorted_result.sort_by(|a, b| a.name.cmp(&b.name));

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
        one_brand: &vec![Brand::new("Mora Radunz".to_owned())],
        multiple_brands: &vec![Brand::new("Otto Shuff".to_owned()), Brand::new("Signe Dadlani".to_owned()), Brand::new("Randal Tuong".to_owned())],
    }

    #[tokio::test]
    async fn add_new_brand_given_empty_repository() {
        let brand: Brand = Brand::new("New Brand".into());
        let mut repository: BrandRepositoryInMemoryImpl = given_empty_repository();

        let result: Result<Brand, BrandRepositoryError> = repository.create(&brand).await;
        let expected: Result<Brand, BrandRepositoryError> = Ok(brand.clone());

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }

    #[tokio::test]
    async fn add_new_brand_given_full_repository() {
        let brand: Brand = Brand::new("New Brand".into());
        let mut repository: BrandRepositoryInMemoryImpl = given_repository_with(vec![
            given_new_brand(),
            given_new_brand(),
            given_new_brand(),
        ]);

        let result: Result<Brand, BrandRepositoryError> = repository.create(&brand).await;
        let expected: Result<Brand, BrandRepositoryError> = Ok(brand);

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }

    #[tokio::test]
    async fn add_existing_brand_given_full_repository() {
        let existing_brand: Brand = Brand::new("Existing Brand".to_owned());

        let mut repository: BrandRepositoryInMemoryImpl = given_repository_with(vec![
            given_new_brand(),
            existing_brand.clone(),
            given_new_brand(),
        ]);

        let result: Result<Brand, BrandRepositoryError> = repository.create(&existing_brand).await;
        let expected: Result<Brand, BrandRepositoryError> =
            Err(BrandRepositoryError::BrandAlreadyExists);

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }
}
