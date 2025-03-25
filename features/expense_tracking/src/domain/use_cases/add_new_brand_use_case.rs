use crate::domain::{entities::Brand, repositories::BrandRepository};

#[derive(Debug, Clone, PartialEq)]
struct AddNewBrandUseCase {}

impl AddNewBrandUseCase {
    fn new(brand_repository: &impl BrandRepository) -> Self {
        Self {}
    }

    async fn execute(&self, name: String) -> Result<Brand, AddNewBrandUseCaseError> {
        Err(AddNewBrandUseCaseError::InvalidName(format!(
            "The name '{}' is not valid",
            name
        )))
    }
}

#[derive(Debug, PartialEq)]
enum AddNewBrandUseCaseError {
    UnableToSaveBrand(String),
    InvalidName(String),
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::domain::{
        entities::Brand,
        repositories::{BrandRepository, BrandRepositoryError},
    };
    use tokio;

    use super::{AddNewBrandUseCase, AddNewBrandUseCaseError};

    #[tokio::test]
    async fn invalid_name() {
        let brand_repository: BrandRepositoryMockImplementation =
            BrandRepositoryMockImplementation::new(
                Err(BrandRepositoryError::Generic(
                    "Vitae erat lacus nam auctor tempor proin imperdiet purus aliquam sed"
                        .to_string(),
                )),
                Err(BrandRepositoryError::Generic(
                    "Vestibulum dui sed ultricies tristique".to_string(),
                )),
            );

        let use_case: AddNewBrandUseCase = AddNewBrandUseCase::new(&brand_repository);

        let result: Result<Brand, AddNewBrandUseCaseError> = use_case.execute("".to_string()).await;
        assert_eq!(
            result,
            Err(AddNewBrandUseCaseError::InvalidName(
                "The name '' is not valid".to_string()
            ))
        );

        let result: Result<Brand, AddNewBrandUseCaseError> =
            use_case.execute("            ".to_string()).await;
        assert_eq!(
            result,
            Err(AddNewBrandUseCaseError::InvalidName(
                "The name '            ' is not valid".to_string()
            ))
        );
    }

    #[derive(Debug, Clone, PartialEq)]
    struct BrandRepositoryMockImplementation {
        create_or_update_result: Result<Option<Brand>, BrandRepositoryError>,
        retrieve_all_result: Result<Vec<Brand>, BrandRepositoryError>,
    }

    impl BrandRepositoryMockImplementation {
        fn new(
            create_or_update_result: Result<Option<Brand>, BrandRepositoryError>,
            retrieve_all_result: Result<Vec<Brand>, BrandRepositoryError>,
        ) -> Self {
            Self {
                create_or_update_result,
                retrieve_all_result,
            }
        }
    }

    #[async_trait]
    impl BrandRepository for BrandRepositoryMockImplementation {
        async fn create_or_update(
            &mut self,
            brand: &Brand,
        ) -> Result<Option<Brand>, BrandRepositoryError> {
            self.create_or_update_result.clone()
        }

        async fn retrieve_all(&self) -> Result<Vec<Brand>, BrandRepositoryError> {
            self.retrieve_all_result.clone()
        }
    }
}
