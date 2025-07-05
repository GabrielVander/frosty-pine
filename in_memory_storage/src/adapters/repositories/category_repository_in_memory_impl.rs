use std::collections::HashMap;

use async_trait::async_trait;
use uuid_b64::UuidB64;

use expense_tracking::domain::{
    entities::Category,
    repositories::{CategoryRepository, CategoryRepositoryError},
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CategoryRepositoryInMemoryImpl {
    hash_map: HashMap<UuidB64, Category>,
}

impl CategoryRepositoryInMemoryImpl {
    pub fn new(hash_map: HashMap<UuidB64, Category>) -> Self {
        Self { hash_map }
    }
}

#[async_trait]
impl CategoryRepository for CategoryRepositoryInMemoryImpl {
    async fn create_or_update(
        &mut self,
        category: &Category,
    ) -> Result<Option<Category>, CategoryRepositoryError> {
        Ok(self.hash_map.insert(category.id, category.clone()))
    }

    async fn retrieve_all(&self) -> Result<Vec<Category>, CategoryRepositoryError> {
        Ok(self.hash_map.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use uuid_b64::UuidB64;

    use super::CategoryRepositoryInMemoryImpl;
    use expense_tracking::domain::{
        entities::Category,
        repositories::{CategoryRepository, CategoryRepositoryError},
    };

    fn given_empty_repository() -> CategoryRepositoryInMemoryImpl {
        given_repository_with(Vec::new())
    }

    fn given_repository_with(categorys: Vec<Category>) -> CategoryRepositoryInMemoryImpl {
        CategoryRepositoryInMemoryImpl::new(
            categorys
                .into_iter()
                .map(|b| (b.id, b))
                .collect::<HashMap<UuidB64, Category>>(),
        )
    }

    fn given_new_category() -> Category {
        Category::new(None, String::default())
    }

    macro_rules! retrieve_all {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[tokio::test]
            async fn $name() {
                let categorys: &Vec<Category> = $value;

                let repository: CategoryRepositoryInMemoryImpl = given_repository_with(categorys.clone());

                let result: Result<Vec<Category>, CategoryRepositoryError> =
                    repository.retrieve_all().await;

                assert!(result.is_ok(), "Expected Ok, got {:?}", result);

                let mut expected_categorys: Vec<Category> = categorys.clone();
                expected_categorys.sort_by(|a, b| a.id.cmp(&b.id));

                let mut sorted_result: Vec<Category> = result.unwrap();
                sorted_result.sort_by(|a, b| a.id.cmp(&b.id));

                assert_eq!(
                    sorted_result, expected_categorys,
                    "Expected {:?}, but got {:?}",
                    expected_categorys, sorted_result
                )
            }
        )*
        }
    }

    retrieve_all! {
        no_categorys: &vec![],
        one_category: &vec![given_new_category()],
        multiple_categorys: &vec![given_new_category(), given_new_category(), given_new_category()],
    }

    #[tokio::test]
    async fn create_or_update_should_add_new_category_given_empty_repository() {
        let category: Category = Category::new(None, "New Category".into());
        let mut repository: CategoryRepositoryInMemoryImpl = given_empty_repository();

        let result: Result<Option<Category>, CategoryRepositoryError> =
            repository.create_or_update(&category).await;
        let expected: Result<Option<Category>, CategoryRepositoryError> = Ok(None);

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }

    #[tokio::test]
    async fn create_or_update_should_add_new_category_given_full_repository() {
        let category: Category = Category::new(None, "New Category".into());
        let mut repository: CategoryRepositoryInMemoryImpl = given_repository_with(vec![
            given_new_category(),
            given_new_category(),
            given_new_category(),
        ]);

        let result: Result<Option<Category>, CategoryRepositoryError> =
            repository.create_or_update(&category).await;
        let expected: Result<Option<Category>, CategoryRepositoryError> = Ok(None);

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }

    #[tokio::test]
    async fn create_or_update_should_update_category_given_full_repository() {
        let old_category: Category = Category::new(None, "New Category".into());
        let updated_category: Category =
            Category::new(Some(old_category.id), "New Updated Category".into());

        let mut repository: CategoryRepositoryInMemoryImpl = given_repository_with(vec![
            given_new_category(),
            old_category.clone(),
            given_new_category(),
        ]);

        let result: Result<Option<Category>, CategoryRepositoryError> =
            repository.create_or_update(&updated_category).await;
        let expected: Result<Option<Category>, CategoryRepositoryError> = Ok(Some(old_category));

        assert_eq!(
            result, expected,
            "Expected {:?}, but got {:?}",
            expected, result
        );
    }
}
