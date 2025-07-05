use expense_tracking::domain::{
    entities::Brand,
    use_cases::{AddNewBrandInteractorError, AddNewBrandOutputPort},
};

use super::models::brand_display_model::BrandDisplayModel;

pub struct FlutterPresenter {}

impl FlutterPresenter {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new() -> FlutterPresenter {
        Self {}
    }
}

impl AddNewBrandOutputPort<Result<BrandDisplayModel, String>> for FlutterPresenter {
    fn apply(&self, input: Result<Brand, AddNewBrandInteractorError>) -> Result<BrandDisplayModel, String> {
        input.map(|brand| brand.into()).map_err(|e| match e {
            AddNewBrandInteractorError::UnableToSaveBrand(_) => "unableToSaveBrand".to_owned(),
            AddNewBrandInteractorError::BrandAlreadyExists => "brandAlreadyExists".to_owned(),
            AddNewBrandInteractorError::InvalidName(_) => "invalidName".to_owned(),
        })
    }
}
