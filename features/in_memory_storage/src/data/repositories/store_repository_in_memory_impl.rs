use std::collections::HashMap;

use async_trait::async_trait;
use uuid_b64::UuidB64;

use expense_tracking::domain::{
    entities::Store,
    repositories::{StoreRepository, StoreRepositoryError},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StoreRepositoryInMemoryImpl {
    hash_map: HashMap<UuidB64, Store>,
}

impl StoreRepositoryInMemoryImpl {
    pub fn new(hash_map: HashMap<UuidB64, Store>) -> Self {
        Self { hash_map }
    }
}

#[async_trait]
impl StoreRepository for StoreRepositoryInMemoryImpl {
    async fn create_or_update(
        &mut self,
        store: &Store,
    ) -> Result<Option<Store>, StoreRepositoryError> {
        Ok(self.hash_map.insert(store.id, store.clone()))
    }

    async fn retrieve_all(&self) -> Result<Vec<Store>, StoreRepositoryError> {
        Ok(self.hash_map.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use uuid_b64::UuidB64;

    use super::StoreRepositoryInMemoryImpl;
    use expense_tracking::domain::{
        entities::Store,
        repositories::{StoreRepository, StoreRepositoryError},
    };

    fn given_empty_repository() -> StoreRepositoryInMemoryImpl {
        given_repository_with(Vec::new())
    }

    fn given_repository_with(stores: Vec<Store>) -> StoreRepositoryInMemoryImpl {
        StoreRepositoryInMemoryImpl::new(
            stores
                .into_iter()
                .map(|b| (b.id, b))
                .collect::<HashMap<UuidB64, Store>>(),
        )
    }

    fn given_new_store() -> Store {
        Store::new(None, String::default())
    }

    macro_rules! retrieve_all {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[tokio::test]
            async fn $name() {
                let stores: &Vec<Store> = $value;

                let repository: StoreRepositoryInMemoryImpl = given_repository_with(stores.clone());

                let result: Result<Vec<Store>, StoreRepositoryError> =
                    repository.retrieve_all().await;

                assert!(result.is_ok(), "Expected Ok, got {:?}", result);

                let mut expected_stores: Vec<Store> = stores.clone();
                expected_stores.sort_by(|a, b| a.id.cmp(&b.id));

                let mut sorted_result: Vec<Store> = result.unwrap();
                sorted_result.sort_by(|a, b| a.id.cmp(&b.id));

                assert_eq!(
                    sorted_result, expected_stores,
                    "Expected {:?}, but got {:?}",
                    expected_stores, sorted_result
                )
            }
        )*
        }
    }

    retrieve_all! {
        no_stores: &vec![],
        one_store: &vec![given_new_store()],
        multiple_stores: &vec![given_new_store(), given_new_store(), given_new_store()],
    }

    #[tokio::test]
    async fn create_or_update_should_add_new_store_given_empty_repository() {
        let store: Store = Store::new(None, "New Store".into());
        let mut repository: StoreRepositoryInMemoryImpl = given_empty_repository();

        let result: Result<Option<Store>, StoreRepositoryError> =
            repository.create_or_update(&store).await;
        let expected: Result<Option<Store>, StoreRepositoryError> = Ok(None);

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }

    #[tokio::test]
    async fn create_or_update_should_add_new_store_given_full_repository() {
        let store: Store = Store::new(None, "New Store".into());
        let mut repository: StoreRepositoryInMemoryImpl = given_repository_with(vec![
            given_new_store(),
            given_new_store(),
            given_new_store(),
        ]);

        let result: Result<Option<Store>, StoreRepositoryError> =
            repository.create_or_update(&store).await;
        let expected: Result<Option<Store>, StoreRepositoryError> = Ok(None);

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }

    #[tokio::test]
    async fn create_or_update_should_update_store_given_full_repository() {
        let old_store: Store = Store::new(None, "New Store".into());
        let updated_store: Store = Store::new(Some(old_store.id), "New Updated Store".into());

        let mut repository: StoreRepositoryInMemoryImpl = given_repository_with(vec![
            given_new_store(),
            old_store.clone(),
            given_new_store(),
        ]);

        let result: Result<Option<Store>, StoreRepositoryError> =
            repository.create_or_update(&updated_store).await;
        let expected: Result<Option<Store>, StoreRepositoryError> = Ok(Some(old_store));

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }
}
