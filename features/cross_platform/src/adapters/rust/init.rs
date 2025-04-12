use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use expense_tracking::domain::{
    entities::Brand,
    repositories::BrandRepository,
    use_cases::{AddNewBrandUseCase, RetrieveAllBrandsUseCase},
};
use in_memory_storage::application::repositories::BrandRepositoryInMemoryImpl;

use super::generated::RustOpaque;

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn create_brand_repository_in_memory_impl(initial_data: Vec<Brand>) -> RustOpaque<Arc<dyn BrandRepository>> {
    let data: Arc<Mutex<HashMap<String, Brand>>> = Arc::new(Mutex::new(
        initial_data
            .into_iter()
            .map(|b| (b.name.clone(), b.clone()))
            .collect::<HashMap<String, Brand>>(),
    ));

    RustOpaque::new(Arc::new(BrandRepositoryInMemoryImpl::new(data)))
}

pub fn create_in_memory_add_new_brand_use_case(brand_repository: RustOpaque<Arc<dyn BrandRepository>>) -> RustOpaque<AddNewBrandUseCase> {
    RustOpaque::new(AddNewBrandUseCase::new(Arc::clone(&brand_repository)))
}

pub fn create_in_memory_retrieve_all_brands_use_case(
    brand_repository: RustOpaque<Arc<dyn BrandRepository>>,
) -> RustOpaque<RetrieveAllBrandsUseCase> {
    RustOpaque::new(RetrieveAllBrandsUseCase::new(Arc::clone(&brand_repository)))
}
