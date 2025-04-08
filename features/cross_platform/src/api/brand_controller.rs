use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use expense_tracking::domain::entities::Brand;
use expense_tracking::domain::repositories::BrandRepository;
use expense_tracking::domain::use_cases::{AddNewBrandUseCase, AddNewBrandUseCaseError, RetrieveAllBrandsUseCase, RetrieveAllBrandsUseCaseError};
use in_memory_storage::application::repositories::BrandRepositoryInMemoryImpl;
use tokio::runtime::Runtime;

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

pub fn create_brands_in_memory_data() -> RustOpaque<Arc<Mutex<HashMap<String, Brand>>>> {
    RustOpaque::new(Arc::new(Mutex::new(HashMap::new())))
}

pub fn execute_add_new_brand_use_case_in_memory(data: RustOpaque<Arc<Mutex<HashMap<String, Brand>>>>, name: String) -> Result<BrandModel, String> {
    let tokio_runtime: Runtime = Runtime::new().unwrap();

    let brand_repository = BrandRepositoryInMemoryImpl::new(Arc::clone(&data));

    let add_new_brand_use_case = AddNewBrandUseCase::new(Box::new(brand_repository));

    tokio_runtime.block_on(async {
        add_new_brand_use_case
            .execute(name)
            .await
            .map(|brand| brand.into())
            .map_err(|e: AddNewBrandUseCaseError| format!("{:?}", e))
    })
}

pub fn execute_retrieve_all_brands_use_case(data: RustOpaque<Arc<Mutex<HashMap<String, Brand>>>>) -> Result<Vec<BrandModel>, String> {
    let tokio_runtime: Runtime = Runtime::new().unwrap();
    let brand_repository = BrandRepositoryInMemoryImpl::new(Arc::clone(&data));
    let retrieve_all_brands_use_case = RetrieveAllBrandsUseCase::new(Box::new(brand_repository));

    tokio_runtime.block_on(async {
        retrieve_all_brands_use_case
            .execute()
            .await
            .map(|brands: Vec<Brand>| brands.into_iter().map(|brand| brand.into()).collect())
            .map_err(|e: RetrieveAllBrandsUseCaseError| format!("{:?}", e))
    })
}

