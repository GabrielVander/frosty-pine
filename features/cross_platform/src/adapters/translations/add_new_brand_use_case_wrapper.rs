use std::sync::Arc;

use expense_tracking::domain::{
    entities::Brand,
    repositories::BrandRepository,
    use_cases::{AddNewBrandUseCase, AddNewBrandUseCaseError},
};

use super::frb_generated::RustOpaque;

pub struct AddNewBrandUseCaseWrapper(Arc<AddNewBrandUseCase>);

impl AddNewBrandUseCaseWrapper {
    pub fn new(brand_repository: RustOpaque<Arc<dyn BrandRepository>>) -> Self {
        Self(Arc::new(AddNewBrandUseCase::new(Arc::clone(&brand_repository))))
    }
    pub async fn execute(&self, name: String) -> Result<Brand, AddNewBrandUseCaseError> {
        self.0.execute(name).await
    }
}
