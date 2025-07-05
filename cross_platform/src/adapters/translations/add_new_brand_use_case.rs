use std::sync::Arc;

use expense_tracking::domain::{
    repositories::BrandRepository,
    use_cases::{AddNewBrandInteractor, AddNewBrandOutputPort},
};

use crate::adapters::presenters::{flutter_presenter::FlutterPresenter, models::brand_display_model::BrandDisplayModel};

use super::frb_generated::RustOpaque;

pub struct AddNewBrandUseCase(Arc<AddNewBrandInteractor<Result<BrandDisplayModel, String>>>);

impl AddNewBrandUseCase {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(brand_repository: RustOpaque<Arc<dyn BrandRepository>>, flutter_presenter: Arc<FlutterPresenter>) -> Self {
        let presenter: Arc<dyn AddNewBrandOutputPort<Result<BrandDisplayModel, String>>> = flutter_presenter;

        Self(Arc::new(AddNewBrandInteractor::new(
            Arc::clone(&brand_repository),
            Arc::clone(&presenter),
        )))
    }

    pub async fn execute(&self, name: String) -> Result<BrandDisplayModel, String> {
        self.0.execute(name).await
    }
}
