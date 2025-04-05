use crate::domain::{
    entities::Brand,
    repositories::{BrandRepository, BrandRepositoryError},
};

#[derive(Debug, Clone)]
struct AddNewBrandUseCase<'a> {
    brand_repository: &'a dyn BrandRepository,
}

impl<'a> AddNewBrandUseCase<'a> {
    fn new(brand_repository: &'a dyn BrandRepository) -> Self {
        Self { brand_repository }
    }

    async fn execute(&self, name: String) -> Result<Brand, AddNewBrandUseCaseError> {
        if name.trim().is_empty() {
            return Err(AddNewBrandUseCaseError::InvalidName(format!(
                "The name '{}' is not valid",
                name
            )));
        }

        let brand: Brand = Brand::new(name);

        self.brand_repository
            .create(&brand)
            .await
            .map_err(|e| e.into())
    }
}

#[derive(Debug, PartialEq)]
enum AddNewBrandUseCaseError {
    InvalidName(String),
    BrandAlreadyExists,
    UnableToSaveBrand(String),
}

impl From<BrandRepositoryError> for AddNewBrandUseCaseError {
    fn from(value: BrandRepositoryError) -> Self {
        match value {
            BrandRepositoryError::BrandAlreadyExists => AddNewBrandUseCaseError::BrandAlreadyExists,
            BrandRepositoryError::UnableToSaveBrand(details) => {
                AddNewBrandUseCaseError::UnableToSaveBrand(details)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use std::fmt::Debug;
    use crate::domain::{
        entities::Brand,
        repositories::{BrandRepository, BrandRepositoryError},
    };
    use tokio;

    use super::{AddNewBrandUseCase, AddNewBrandUseCaseError};

    #[tokio::test]
    async fn should_fail_if_given_invalid_name() {
        let brand_repository: BrandRepositoryMockImplementation =
            BrandRepositoryMockImplementation::none();

        let use_case: AddNewBrandUseCase = AddNewBrandUseCase::new(&brand_repository);

        let result: Result<Brand, AddNewBrandUseCaseError> = use_case.execute("".to_owned()).await;
        assert_eq!(
            result,
            Err(AddNewBrandUseCaseError::InvalidName(
                "The name '' is not valid".to_owned()
            ))
        );

        let result: Result<Brand, AddNewBrandUseCaseError> =
            use_case.execute("            ".to_owned()).await;
        assert_eq!(
            result,
            Err(AddNewBrandUseCaseError::InvalidName(
                "The name '            ' is not valid".to_owned()
            ))
        );
    }

    #[tokio::test]
    async fn should_fail_if_brand_already_exists() {
        let brand_repository: BrandRepositoryMockImplementation =
            BrandRepositoryMockImplementation::on_create_returns(Err(BrandRepositoryError::BrandAlreadyExists)
            );

        let use_case: AddNewBrandUseCase = AddNewBrandUseCase::new(&brand_repository);

        let result: Result<Brand, AddNewBrandUseCaseError> =
            use_case.execute("Shena Glore".to_owned()).await;
        assert_eq!(result, Err(AddNewBrandUseCaseError::BrandAlreadyExists));
    }

    #[tokio::test]
    async fn should_fail_if_unable_to_save() {
        let unable_to_save_details: String =
            "Vitae erat lacus nam auctor tempor proin imperdiet purus aliquam sed".to_owned();
        let brand_repository: BrandRepositoryMockImplementation =
            BrandRepositoryMockImplementation::on_create_returns(Err(BrandRepositoryError::UnableToSaveBrand(unable_to_save_details.clone())));

        let use_case: AddNewBrandUseCase = AddNewBrandUseCase::new(&brand_repository);

        let result: Result<Brand, AddNewBrandUseCaseError> =
            use_case.execute("Harris Bovia".to_owned()).await;
        assert_eq!(
            result,
            Err(AddNewBrandUseCaseError::UnableToSaveBrand(
                unable_to_save_details.clone()
            ))
        );
    }

    #[tokio::test]
    async fn should_return_brand_if_success() {
        let target_name: String = "Jeffery Murilla".to_owned();
        let expected_brand: Brand = Brand::new(target_name.clone());
        let brand_repository: BrandRepositoryMockImplementation =
            BrandRepositoryMockImplementation::on_create_returns(Ok(expected_brand.clone()));

        let use_case: AddNewBrandUseCase = AddNewBrandUseCase::new(&brand_repository);

        let result: Result<Brand, AddNewBrandUseCaseError> = use_case.execute(target_name).await;
        assert_eq!(result, Ok(expected_brand));
    }

    #[derive(Debug)]
    struct BrandRepositoryMockImplementation {
        on_create: Option<Result<Brand, BrandRepositoryError>>,
        on_retrieve_all:  Option<Result<Vec<Brand>, BrandRepositoryError>>,
    }

    impl BrandRepositoryMockImplementation {
        fn none() -> Self {
            Self {
                on_create: None,
                on_retrieve_all: None,
            }
        }

        fn on_create_returns(result: Result<Brand, BrandRepositoryError>) -> Self {
            Self {
                on_create: Some(result),
                on_retrieve_all: None,
            }
        }
    }

    #[async_trait]
    impl BrandRepository for BrandRepositoryMockImplementation {
        async fn create(&self, _: &Brand) -> Result<Brand, BrandRepositoryError> {
            self.on_create.clone().unwrap_or_else(|| todo!())
        }

        async fn retrieve_all(&self) -> Result<Vec<Brand>, BrandRepositoryError> {
            self.on_retrieve_all.clone().unwrap_or_else(|| todo!())
        }
    }

}
