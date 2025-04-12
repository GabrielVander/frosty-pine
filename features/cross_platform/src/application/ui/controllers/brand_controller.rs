use expense_tracking::domain::{
    entities::Brand,
    use_cases::{AddNewBrandUseCase, AddNewBrandUseCaseError, RetrieveAllBrandsUseCase, RetrieveAllBrandsUseCaseError},
};

use crate::{application::ui::models::brand_model::BrandModel, frb_generated::RustOpaque};

pub struct BrandsController {
    pub add_new_brand_use_case: RustOpaque<AddNewBrandUseCase>,
    pub retrieve_all_brands_use_case: RustOpaque<RetrieveAllBrandsUseCase>,
}

impl BrandsController {
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

    pub async fn add_new_brand(&self, name: String) -> Result<BrandModel, String> {
        self.add_new_brand_use_case
            .execute(name)
            .await
            .map(|brand| brand.into())
            .map_err(|e: AddNewBrandUseCaseError| format!("{:?}", e))
    }

    pub async fn retrieve_all_brands(&self) -> Result<Vec<BrandModel>, String> {
        self.retrieve_all_brands_use_case
            .execute()
            .await
            .map(|brands: Vec<Brand>| brands.into_iter().map(|brand| brand.into()).collect())
            .map_err(|e: RetrieveAllBrandsUseCaseError| format!("{:?}", e))
    }
}
