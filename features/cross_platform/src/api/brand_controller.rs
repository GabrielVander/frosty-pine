use std::collections::HashMap;

use expense_tracking::domain::entities::Brand;
use expense_tracking::domain::use_cases::{AddNewBrandUseCase, AddNewBrandUseCaseError};

use super::models::BrandModel;

pub async fn add_new_brand(name: String) -> Result<BrandModel, AddNewBrandUseCaseError> {
    use in_memory_storage::application::repositories::BrandRepositoryInMemoryImpl;
    let brand_in_memory_repository: Box<BrandRepositoryInMemoryImpl> =
        Box::new(BrandRepositoryInMemoryImpl::new(HashMap::new()));
    let mut use_case: AddNewBrandUseCase = AddNewBrandUseCase::new(brand_in_memory_repository);

    use_case.execute(name).await
}
