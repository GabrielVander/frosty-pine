use std::sync::Arc;

use expense_tracking::domain::{
    entities::Brand,
    repositories::BrandRepository,
    use_cases::{AddNewBrandUseCase, AddNewBrandUseCaseError, UseCaseInputPort, UseCaseOutputPort},
};

use crate::adapters::presenters::{flutter_presenter::FlutterPresenter, models::brand_display_model::BrandDisplayModel};

use super::frb_generated::RustOpaque;

// pub struct AddNewBrandUseCaseWrapper(pub Arc<AddNewBrandUseCase>);
//
// impl AddNewBrandUseCase {
//     pub fn new(brand_repository: RustOpaque<Arc<dyn BrandRepository>>) -> RustOpaque<AddNewBrandUseCaseWrapper> {
//         let for_add_new_brand_use_case: Arc<
//             dyn UseCaseOutputPort<Input = Result<Brand, AddNewBrandUseCaseError>, Output = Result<BrandDisplayModel, String>>,
//         > = FlutterPresenter::new();
//
//         RustOpaque::new(Self(Arc::new(AddNewBrandUseCase::new(
//             Arc::clone(&brand_repository),
//             Arc::clone(&for_add_new_brand_use_case),
//         ))))
//     }
//     pub async fn execute(&self, name: String) -> Result<BrandDisplayModel, String> {
//         self.0.execute(name).await
//     }
// }
