use expense_tracking::domain::{
    entities::Brand,
    use_cases::{AddNewBrandUseCase, AddNewBrandUseCaseError, RetrieveAllBrandsUseCase, RetrieveAllBrandsUseCaseError},
};

use crate::adapters::{models::brand_display_model::BrandDisplayModel, rust::generated::RustOpaque};

pub struct BrandPresenter {
    pub add_new_brand_use_case: RustOpaque<AddNewBrandUseCase>,
    pub retrieve_all_brands_use_case: RustOpaque<RetrieveAllBrandsUseCase>,
}

impl BrandPresenter {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(
        add_new_brand_use_case: RustOpaque<AddNewBrandUseCase>,
        retrieve_all_brands_use_case: RustOpaque<RetrieveAllBrandsUseCase>,
    ) -> Self {
        Self {
            add_new_brand_use_case,
            retrieve_all_brands_use_case,
        }
    }

    pub async fn add_new_brand(&self, name: String) -> Result<BrandDisplayModel, String> {
        self.add_new_brand_use_case
            .execute(name)
            .await
            .map(|brand| brand.into())
            .map_err(|e: AddNewBrandUseCaseError| format!("{:?}", e))
    }

    pub async fn retrieve_all_brands(&self) -> Result<Vec<BrandDisplayModel>, String> {
        self.retrieve_all_brands_use_case
            .execute()
            .await
            .map(|brands: Vec<Brand>| brands.into_iter().map(|brand| brand.into()).collect())
            .map_err(|e: RetrieveAllBrandsUseCaseError| format!("{:?}", e))
    }
}
