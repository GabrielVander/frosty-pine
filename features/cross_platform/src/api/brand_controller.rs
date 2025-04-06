use expense_tracking::domain::entities::Brand;
use expense_tracking::domain::use_cases::AddNewBrandUseCase;

#[derive(Debug)]
pub struct BrandController {
    add_brand_use_case: AddNewBrandUseCase,
}

impl BrandController {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(add_brand_use_case: AddNewBrandUseCase) -> Self {
        Self {
            add_brand_use_case
        }
    }

    pub async fn add_new_brand(&mut self, name: String) -> Result<BrandModel, String> {
        self.add_brand_use_case
            .execute(name)
            .await
            .map(|brand: Brand| brand.into())
            .map_err(|e| format!("{:?}", e))
    }

    // pub async fn retrieve_all_brands(&self) -> Result<Vec<BrandModel>, String> {
    //     self.brand_repository.retrieve_all
    // }
}

pub struct BrandModel { pub name: String }

impl From<Brand> for BrandModel {
    fn from(value: Brand) -> Self {
        Self { name: value.name }
    }
}

