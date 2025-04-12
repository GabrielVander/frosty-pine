use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;

use expense_tracking::domain::{
    entities::Brand,
    repositories::{BrandRepository, BrandRepositoryCreateError, BrandRepositoryRetrieveAllError},
};

#[derive(Debug)]
pub struct BrandRepositoryInMemoryImpl {
    hash_map: Arc<Mutex<HashMap<String, Brand>>>,
}

impl BrandRepositoryInMemoryImpl {
    pub fn new(hash_map: Arc<Mutex<HashMap<String, Brand>>>) -> Self {
        Self { hash_map }
    }
}

#[async_trait]
impl BrandRepository for BrandRepositoryInMemoryImpl {
    async fn create(&self, brand: &Brand) -> Result<Brand, BrandRepositoryCreateError> {
        if self.hash_map.lock().unwrap().contains_key(&brand.name) {
            return Err(BrandRepositoryCreateError::BrandAlreadyExists);
        }

        self.hash_map.lock().unwrap().insert(brand.name.clone(), brand.clone());
        Ok(brand.clone())
    }

    async fn retrieve_all(&self) -> Result<Vec<Brand>, BrandRepositoryRetrieveAllError> {
        Ok(self.hash_map.lock().unwrap().values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use super::BrandRepositoryInMemoryImpl;
    use expense_tracking::domain::{
        entities::Brand,
        repositories::{BrandRepository, BrandRepositoryCreateError, BrandRepositoryRetrieveAllError},
    };

    fn given_new_brand() -> Brand {
        Brand::new(String::default())
    }

    macro_rules! retrieve_all {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[tokio::test]
            async fn $name() {
                let brands: &Vec<Brand> = $value;

                let data: Arc<Mutex<HashMap<String, Brand>>> = Arc::new(Mutex::new(brands
                    .into_iter()
                    .map(|b| (b.name.clone(), b.clone()))
                    .collect::<HashMap<String, Brand>>()));

                let repository: BrandRepositoryInMemoryImpl = BrandRepositoryInMemoryImpl::new(Arc::clone(&data));

                let result: Result<Vec<Brand>, BrandRepositoryRetrieveAllError> =
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
        let data: Arc<Mutex<HashMap<String, Brand>>> = Arc::new(Mutex::new(HashMap::new()));
        let repository: BrandRepositoryInMemoryImpl = BrandRepositoryInMemoryImpl::new(Arc::clone(&data));

        let result: Result<Brand, BrandRepositoryCreateError> = repository.create(&brand).await;
        let expected: Result<Brand, BrandRepositoryCreateError> = Ok(brand.clone());

        assert_eq!(result, expected, "Expected {:?}, but got {:?}", expected, result);
    }

    #[tokio::test]
    async fn add_new_brand_given_full_repository() {
        let data: Arc<Mutex<HashMap<String, Brand>>> = Arc::new(Mutex::new(
            vec![given_new_brand(), given_new_brand(), given_new_brand()]
                .into_iter()
                .map(|b| (b.name.clone(), b.clone()))
                .collect::<HashMap<String, Brand>>(),
        ));

        let brand: Brand = Brand::new("New Brand".into());

        let repository: BrandRepositoryInMemoryImpl = BrandRepositoryInMemoryImpl::new(Arc::clone(&data));

        let result: Result<Brand, BrandRepositoryCreateError> = repository.create(&brand).await;
        let expected: Result<Brand, BrandRepositoryCreateError> = Ok(brand);

        assert_eq!(result, expected, "Expected {:?}, but got {:?}", expected, result);
    }

    #[tokio::test]
    async fn add_existing_brand_given_full_repository() {
        let existing_brand: Brand = Brand::new("Existing Brand".to_owned());

        let data: Arc<Mutex<HashMap<String, Brand>>> = Arc::new(Mutex::new(
            vec![given_new_brand(), existing_brand.clone(), given_new_brand()]
                .into_iter()
                .map(|b| (b.name.clone(), b.clone()))
                .collect::<HashMap<String, Brand>>(),
        ));

        let repository: BrandRepositoryInMemoryImpl = BrandRepositoryInMemoryImpl::new(Arc::clone(&data));

        let result: Result<Brand, BrandRepositoryCreateError> = repository.create(&existing_brand).await;
        let expected: Result<Brand, BrandRepositoryCreateError> = Err(BrandRepositoryCreateError::BrandAlreadyExists);

        assert_eq!(result, expected, "Expected {:?}, but got {:?}", expected, result);
    }
}
