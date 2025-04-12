use std::collections::HashMap;

use async_trait::async_trait;
use uuid_b64::UuidB64;

use expense_tracking::domain::{
    entities::Product,
    repositories::{ProductRepository, ProductRepositoryError},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProductRepositoryInMemoryImpl {
    hash_map: HashMap<UuidB64, Product>,
}

impl ProductRepositoryInMemoryImpl {
    pub fn new(hash_map: HashMap<UuidB64, Product>) -> Self {
        Self { hash_map }
    }
}

#[async_trait]
impl ProductRepository for ProductRepositoryInMemoryImpl {
    async fn create_or_update(
        &mut self,
        product: &Product,
    ) -> Result<Option<Product>, ProductRepositoryError> {
        Ok(self.hash_map.insert(product.id, product.clone()))
    }

    async fn retrieve_all(&self) -> Result<Vec<Product>, ProductRepositoryError> {
        Ok(self.hash_map.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use uuid_b64::UuidB64;

    use super::ProductRepositoryInMemoryImpl;
    use expense_tracking::domain::{
        entities::{Brand, Category, Product},
        repositories::{ProductRepository, ProductRepositoryError},
    };

    fn given_empty_repository() -> ProductRepositoryInMemoryImpl {
        given_repository_with(Vec::new())
    }

    fn given_repository_with(products: Vec<Product>) -> ProductRepositoryInMemoryImpl {
        ProductRepositoryInMemoryImpl::new(
            products
                .into_iter()
                .map(|b| (b.id, b))
                .collect::<HashMap<UuidB64, Product>>(),
        )
    }

    fn given_new_brand() -> Brand {
        Brand::new(String::default())
    }

    fn given_new_category() -> Category {
        Category::new(None, String::default())
    }

    fn given_new_product() -> Product {
        Product::new(
            None,
            String::default(),
            given_new_brand(),
            given_new_category(),
        )
    }

    macro_rules! retrieve_all {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[tokio::test]
            async fn $name() {
                let products: &Vec<Product> = $value;

                let repository: ProductRepositoryInMemoryImpl = given_repository_with(products.clone());

                let result: Result<Vec<Product>, ProductRepositoryError> =
                    repository.retrieve_all().await;

                assert!(result.is_ok(), "Expected Ok, got {:?}", result);

                let mut expected_products: Vec<Product> = products.clone();
                expected_products.sort_by(|a, b| a.id.cmp(&b.id));

                let mut sorted_result: Vec<Product> = result.unwrap();
                sorted_result.sort_by(|a, b| a.id.cmp(&b.id));

                assert_eq!(
                    sorted_result, expected_products,
                    "Expected {:?}, but got {:?}",
                    expected_products, sorted_result
                )
            }
        )*
        }
    }

    retrieve_all! {
        no_products: &vec![],
        one_product: &vec![given_new_product()],
        multiple_products: &vec![given_new_product(), given_new_product(), given_new_product()],
    }

    #[tokio::test]
    async fn create_or_update_should_add_new_product_given_empty_repository() {
        let product: Product = Product::new(
            None,
            "New Product".into(),
            given_new_brand(),
            given_new_category(),
        );
        let mut repository: ProductRepositoryInMemoryImpl = given_empty_repository();

        let result: Result<Option<Product>, ProductRepositoryError> =
            repository.create_or_update(&product).await;
        let expected: Result<Option<Product>, ProductRepositoryError> = Ok(None);

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }

    #[tokio::test]
    async fn create_or_update_should_add_new_product_given_full_repository() {
        let product: Product = Product::new(
            None,
            "New Product".into(),
            given_new_brand(),
            given_new_category(),
        );
        let mut repository: ProductRepositoryInMemoryImpl = given_repository_with(vec![
            given_new_product(),
            given_new_product(),
            given_new_product(),
        ]);

        let result: Result<Option<Product>, ProductRepositoryError> =
            repository.create_or_update(&product).await;
        let expected: Result<Option<Product>, ProductRepositoryError> = Ok(None);

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }

    #[tokio::test]
    async fn create_or_update_should_update_product_given_full_repository() {
        let old_product: Product = Product::new(
            None,
            "New Product".into(),
            given_new_brand(),
            given_new_category(),
        );
        let updated_product: Product = Product::new(
            Some(old_product.id),
            "New Updated Product".into(),
            given_new_brand(),
            given_new_category(),
        );

        let mut repository: ProductRepositoryInMemoryImpl = given_repository_with(vec![
            given_new_product(),
            old_product.clone(),
            given_new_product(),
        ]);

        let result: Result<Option<Product>, ProductRepositoryError> =
            repository.create_or_update(&updated_product).await;
        let expected: Result<Option<Product>, ProductRepositoryError> = Ok(Some(old_product));

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }
}
