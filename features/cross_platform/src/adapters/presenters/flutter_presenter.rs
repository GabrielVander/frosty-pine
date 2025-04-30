use std::sync::Arc;

use expense_tracking::domain::use_cases::{AddNewBrandUseCase, AddNewBrandUseCaseError};

pub use expense_tracking::domain::use_cases::UseCaseOutputPort;

use crate::adapters::translations::frb_generated::RustOpaque;

use super::models::brand_display_model::BrandDisplayModel;

pub struct FlutterPresenter {}

// impl FlutterPresenter {
//     #[flutter_rust_bridge::frb(sync)]
//     pub fn new() -> Arc<FlutterPresenter> {
//         Arc::new(Self {})
//     }
// }
//
// impl UseCaseOutputPort for FlutterPresenter {
//     type Input = Result<Brand, AddNewBrandUseCaseError>;
//     type Output = Result<BrandDisplayModel, String>;
//
//     #[flutter_rust_bridge::frb()]
//     fn apply(&self, input: Result<Brand, AddNewBrandUseCaseError>) -> Result<BrandDisplayModel, String> {
//         todo!()
//     }
// }

pub struct BrandFlutterPresenter {
    pub add_new_brand_use_case: RustOpaque<AddNewBrandUseCase>,
}

impl BrandFlutterPresenter {
    pub fn new(add_new_brand_use_case: RustOpaque<AddNewBrandUseCase>) -> Self {
        Self { add_new_brand_use_case }
    }

    pub async fn add_new_brand(&self, name: String) -> Result<BrandDisplayModel, String> {
        self.add_new_brand_use_case
            .execute(name)
            .await
            .map(|brand| brand.into())
            .map_err(|e| match e {
                AddNewBrandUseCaseError::UnableToSaveBrand(_) => "unableToSaveBrand".to_owned(),
                AddNewBrandUseCaseError::BrandAlreadyExists => "brandAlreadyExists".to_owned(),
                AddNewBrandUseCaseError::InvalidName(_) => "invalidName".to_owned(),
            })
    }
}
