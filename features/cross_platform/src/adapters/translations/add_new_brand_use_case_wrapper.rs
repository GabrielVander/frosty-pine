use std::sync::Arc;

use expense_tracking::domain::{
    repositories::BrandRepository,
    use_cases::{AddNewBrandUseCase, AddNewBrandUseCasePresenter},
};

use crate::adapters::presenters::models::brand_display_model::BrandDisplayModel;

use super::frb_generated::RustOpaque;

pub struct AddNewBrandUseCaseWrapper(Arc<AddNewBrandUseCase<Result<BrandDisplayModel, String>>>);

impl AddNewBrandUseCaseWrapper {
    pub fn new(
        brand_repository: RustOpaque<Arc<dyn BrandRepository>>,
        presenter: Arc<AddNewBrandUseCasePresenter<Result<BrandDisplayModel, String>>>,
    ) -> Self {
        Self(Arc::new(AddNewBrandUseCase::new(
            Arc::clone(&brand_repository),
            Arc::clone(&presenter),
        )))
    }
    pub async fn execute(&self, name: String) -> Result<BrandDisplayModel, String> {
        self.0.execute(name).await
    }
}
