use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use expense_tracking::domain::{entities::Brand, repositories::BrandRepository, use_cases::RetrieveAllBrandsUseCase};
use in_memory_storage::adapters::repositories::BrandRepositoryInMemoryImpl;

use crate::adapters::presenters::flutter_presenter::FlutterPresenter;

use super::frb_generated::RustOpaque;

pub struct RustFactory {}

impl RustFactory {
    pub fn brand_repository_in_memory_impl(initial_data: Vec<Brand>) -> RustOpaque<Arc<dyn BrandRepository>> {
        let data: Arc<Mutex<HashMap<String, Brand>>> = Arc::new(Mutex::new(
            initial_data
                .into_iter()
                .map(|b| (b.name.clone(), b.clone()))
                .collect::<HashMap<String, Brand>>(),
        ));

        RustOpaque::new(Arc::new(BrandRepositoryInMemoryImpl::new(data)))
    }

    pub fn retrieve_all_brands_use_case(brand_repository: RustOpaque<Arc<dyn BrandRepository>>) -> RustOpaque<RetrieveAllBrandsUseCase> {
        RustOpaque::new(RetrieveAllBrandsUseCase::new(Arc::clone(&brand_repository)))
    }

    pub fn flutter_presenter() -> Arc<FlutterPresenter> {
        Arc::new(FlutterPresenter::new())
    }
}
