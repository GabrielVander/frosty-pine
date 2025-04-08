use crate::domain::{
    entities::Brand,
    repositories::{BrandRepository, BrandRepositoryRetrieveAllError},
};

#[derive(Debug)]
pub struct RetrieveAllBrandsUseCase {
    brand_repository: Box<dyn BrandRepository>,
}

impl RetrieveAllBrandsUseCase {
    pub fn new(brand_repository: Box<dyn BrandRepository>) -> Self {
        Self { brand_repository }
    }

    pub async fn execute(&self) -> Result<Vec<Brand>, RetrieveAllBrandsUseCaseError> {
        self.brand_repository
            .retrieve_all()
            .await
            .map_err(|e: BrandRepositoryRetrieveAllError| e.into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RetrieveAllBrandsUseCaseError {
    UnableToRetrieveBrands(String),
}

impl From<BrandRepositoryRetrieveAllError> for RetrieveAllBrandsUseCaseError {
    fn from(value: BrandRepositoryRetrieveAllError) -> Self {
        match value {
            BrandRepositoryRetrieveAllError::UnableToRetrieveBrands(info) => {
                RetrieveAllBrandsUseCaseError::UnableToRetrieveBrands(info)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        entities::Brand,
        repositories::{
            BrandRepository, BrandRepositoryCreateError, BrandRepositoryRetrieveAllError,
        },
        use_cases::retrieve_all_brands_use_case::{
            RetrieveAllBrandsUseCase, RetrieveAllBrandsUseCaseError,
        },
    };
    use async_trait::async_trait;

    macro_rules! parameterized_tests {
        ($($name:ident: ($a:expr, $b:expr))*) => {
            $(
                #[tokio::test]
                async fn $name() {
                    let brand_repository_retrieve_all_result: Result<Vec<Brand>, BrandRepositoryRetrieveAllError> = $a;
                    let expected: Result<Vec<Brand>, RetrieveAllBrandsUseCaseError> = $b;

                    let brand_repository: BrandRepositoryMockImplementation = BrandRepositoryMockImplementation::on_retrieve_all_returns(brand_repository_retrieve_all_result);

                    let use_case: RetrieveAllBrandsUseCase = RetrieveAllBrandsUseCase::new(Box::new(brand_repository));

                    let result: Result<Vec<Brand>, RetrieveAllBrandsUseCaseError> = use_case.execute().await;

                    assert_eq!(
                        result, expected,
                        "Expected {:?}, but got {:?}",
                        expected, result
                    )
                }
            )*
        }

    }

    parameterized_tests! {
        should_return_err_if_unable_to_retrieve_brands_default_string: (
            Err(BrandRepositoryRetrieveAllError::UnableToRetrieveBrands(String::default())), Err(RetrieveAllBrandsUseCaseError::UnableToRetrieveBrands(String::default()))
        )
        should_return_err_if_unable_to_retrieve_brands_some_random_string: (
            Err(BrandRepositoryRetrieveAllError::UnableToRetrieveBrands("Ut eros porta velit metus et fringilla".to_owned())), Err(RetrieveAllBrandsUseCaseError::UnableToRetrieveBrands("Ut eros porta velit metus et fringilla".to_owned()))
        )
        should_return_ok_no_brands: (Ok(vec![]), Ok(vec![]))
        should_return_ok_one_brand: (Ok(vec![Brand::new("Anja Askland".to_owned())]), Ok(vec![Brand::new("Anja Askland".to_owned())]))
        should_return_ok_multiple_brands: (Ok(vec![Brand::new("Anja Askland".to_owned()), Brand::new("Loris Duve".to_owned()),Brand::new("Liz Humerickhouse".to_owned())]), Ok(vec![Brand::new("Anja Askland".to_owned()), Brand::new("Loris Duve".to_owned()),Brand::new("Liz Humerickhouse".to_owned())]))
    }

    #[derive(Debug)]
    struct BrandRepositoryMockImplementation {
        on_retrieve_all: Option<Result<Vec<Brand>, BrandRepositoryRetrieveAllError>>,
    }

    impl BrandRepositoryMockImplementation {
        fn on_retrieve_all_returns(
            result: Result<Vec<Brand>, BrandRepositoryRetrieveAllError>,
        ) -> Self {
            Self {
                on_retrieve_all: Some(result),
            }
        }
    }

    #[async_trait]
    impl BrandRepository for BrandRepositoryMockImplementation {
        async fn create(&self, _: &Brand) -> Result<Brand, BrandRepositoryCreateError> {
            todo!()
        }

        async fn retrieve_all(&self) -> Result<Vec<Brand>, BrandRepositoryRetrieveAllError> {
            self.on_retrieve_all.clone().unwrap_or_else(|| todo!())
        }
    }
}
