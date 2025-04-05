use std::collections::HashMap;

use expense_tracking::domain::entities::Brand;
use expense_tracking::domain::repositories::BrandRepository;
use expense_tracking::domain::use_cases::AddNewBrandUseCase;

use super::models::BrandModel;

use in_memory_storage::application::repositories::BrandRepositoryInMemoryImpl;

#[derive(Debug)]
pub struct BrandController {
    add_brand_use_case: AddNewBrandUseCase,
}

impl BrandController {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new() -> Self {
        let brand_repository: Box<dyn BrandRepository + Sync + Send> =
            Box::new(BrandRepositoryInMemoryImpl::new(HashMap::new()));
        Self {
            add_brand_use_case: AddNewBrandUseCase::new(brand_repository),
        }
    }
    pub async fn add_new_brand(&mut self, name: String) -> Result<BrandModel, String> {
        self.add_brand_use_case
            .execute(name)
            .await
            .map(|brand: Brand| brand.into())
            .map_err(|e| format!("{:?}", e))
    }
}
