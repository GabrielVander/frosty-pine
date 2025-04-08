use crate::domain::{
    entities::Brand,
    repositories::{BrandRepository, BrandRepositoryCreateError},
};

#[derive(Debug)]
pub struct AddNewBrandUseCase {
    brand_repository: Box<dyn BrandRepository>,
}

impl AddNewBrandUseCase {
    pub fn new(brand_repository: Box<dyn BrandRepository>) -> Self {
        Self { brand_repository }
    }

    pub async fn execute(&self, name: String) -> Result<Brand, AddNewBrandUseCaseError> {
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
pub enum AddNewBrandUseCaseError {
    InvalidName(String),
    BrandAlreadyExists,
    UnableToSaveBrand(String),
}

impl From<BrandRepositoryCreateError> for AddNewBrandUseCaseError {
    fn from(value: BrandRepositoryCreateError) -> Self {
        match value {
            BrandRepositoryCreateError::BrandAlreadyExists => AddNewBrandUseCaseError::BrandAlreadyExists,
            BrandRepositoryCreateError::UnableToSaveBrand(details) => {
                AddNewBrandUseCaseError::UnableToSaveBrand(details)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::domain::{
        entities::Brand,
        repositories::{BrandRepository, BrandRepositoryCreateError, BrandRepositoryRetrieveAllError},
    };
    use std::fmt::Debug;
    use tokio;

    use super::{AddNewBrandUseCase, AddNewBrandUseCaseError};

    #[tokio::test]
    async fn should_fail_if_given_invalid_name() {
        let  brand_repository: BrandRepositoryMockImplementation = BrandRepositoryMockImplementation::none();

        let  use_case: AddNewBrandUseCase = AddNewBrandUseCase::new(Box::new(brand_repository));

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
        let  brand_repository: BrandRepositoryMockImplementation =
            BrandRepositoryMockImplementation::on_create_returns(Err(
                BrandRepositoryCreateError::BrandAlreadyExists,
            ));

        let  use_case: AddNewBrandUseCase = AddNewBrandUseCase::new(Box::new(brand_repository));

        let result: Result<Brand, AddNewBrandUseCaseError> =
            use_case.execute("Shena Glore".to_owned()).await;
        assert_eq!(result, Err(AddNewBrandUseCaseError::BrandAlreadyExists));
    }

    #[tokio::test]
    async fn should_fail_if_unable_to_save() {
        let unable_to_save_details: String =
            "Vitae erat lacus nam auctor tempor proin imperdiet purus aliquam sed".to_owned();
        let  brand_repository: BrandRepositoryMockImplementation =
            BrandRepositoryMockImplementation::on_create_returns(Err(
                BrandRepositoryCreateError::UnableToSaveBrand(unable_to_save_details.clone()),
            ));

        let  use_case: AddNewBrandUseCase = AddNewBrandUseCase::new(Box::new(brand_repository));

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

        let  use_case: AddNewBrandUseCase = AddNewBrandUseCase::new(Box::new(brand_repository));

        let result: Result<Brand, AddNewBrandUseCaseError> = use_case.execute(target_name).await;
        assert_eq!(result, Ok(expected_brand));
    }

    #[derive(Debug)]
    struct BrandRepositoryMockImplementation {
        on_create: Option<Result<Brand, BrandRepositoryCreateError>>,
    }

    impl BrandRepositoryMockImplementation {
        fn none() -> Self {
            Self {
                on_create: None,
            }
        }

        fn on_create_returns(result: Result<Brand, BrandRepositoryCreateError>) -> Self {
            Self {
                on_create: Some(result),
            }
        }
    }

    #[async_trait]
    impl BrandRepository for BrandRepositoryMockImplementation {
        async fn create(&self, _: &Brand) -> Result<Brand, BrandRepositoryCreateError> {
           self.on_create.clone().unwrap_or_else(|| todo!())
        }

        async fn retrieve_all(&self) -> Result<Vec<Brand>, BrandRepositoryRetrieveAllError> {
            todo!()
        }
    }
}
