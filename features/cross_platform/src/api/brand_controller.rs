use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use expense_tracking::domain::entities::Brand;
use expense_tracking::domain::repositories::BrandRepository;
use expense_tracking::domain::use_cases::{AddNewBrandUseCase, AddNewBrandUseCaseError, RetrieveAllBrandsUseCase, RetrieveAllBrandsUseCaseError};
use in_memory_storage::application::repositories::BrandRepositoryInMemoryImpl;

use crate::frb_generated::RustOpaque;

pub struct BrandModel { pub name: String }

impl From<&Brand> for BrandModel {
    fn from(value: &Brand) -> Self {
        Self { name: value.name.clone() }
    }
}

impl From<Brand> for BrandModel {
    fn from(value: Brand) -> Self {
        Self { name: value.name }
    }
}

pub fn create_brand_repository_in_memory_impl(initial_data: Vec<Brand>) -> RustOpaque<Arc<dyn BrandRepository>> {
    let data: Arc<Mutex<HashMap<String, Brand>>> = Arc::new(Mutex::new(initial_data.into_iter()
                    .map(|b| (b.name.clone(), b.clone()))
                    .collect::<HashMap<String, Brand>>()));

    RustOpaque::new(Arc::new(BrandRepositoryInMemoryImpl::new(data)))
}

pub fn create_in_memory_add_new_brand_use_case(brand_repository: RustOpaque<Arc<dyn BrandRepository>>)-> RustOpaque<AddNewBrandUseCase> {
    RustOpaque::new(AddNewBrandUseCase::new(Arc::clone(&brand_repository)))
}

pub fn create_in_memory_retrieve_all_brands_use_case(brand_repository: RustOpaque<Arc<dyn BrandRepository>>)-> RustOpaque<RetrieveAllBrandsUseCase> {
    RustOpaque::new(RetrieveAllBrandsUseCase::new(Arc::clone(&brand_repository)))
}

#[flutter_rust_bridge::frb(non_opaque)]
pub struct BrandsController {
    pub add_new_brand_use_case: RustOpaque<AddNewBrandUseCase>,
    pub retrieve_all_brands_use_case: RustOpaque<RetrieveAllBrandsUseCase>
}

impl BrandsController {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(add_new_brand_use_case: RustOpaque<AddNewBrandUseCase>, retrieve_all_brands_use_case: RustOpaque<RetrieveAllBrandsUseCase>) -> Self {
        Self {
            add_new_brand_use_case,
            retrieve_all_brands_use_case
        }
    }

    pub async fn add_new_brand(&self, name: String) -> Result<BrandModel, String>{
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
