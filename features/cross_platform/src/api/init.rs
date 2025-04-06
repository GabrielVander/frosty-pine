use std::collections::HashMap;

use expense_tracking::domain::{
    entities::Brand, repositories::BrandRepository, use_cases::AddNewBrandUseCase,
};
use in_memory_storage::application::repositories::BrandRepositoryInMemoryImpl;

pub fn create_brand_repository_in_memory_impl(
    data: HashMap<String, Brand>,
) -> Box<dyn BrandRepository + Sync + Send> {
    Box::new(BrandRepositoryInMemoryImpl::new(data))
}

pub fn create_add_new_brand_use_case(
    brand_repository: Box<dyn BrandRepository + Sync + Send>,
) -> AddNewBrandUseCase {
    AddNewBrandUseCase::new(brand_repository)
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
