use std::collections::HashMap;

use expense_tracking::domain::{
    entities::Store,
    repositories::{StoreRepository, StoreRepositoryError},
};
use in_memory_storage::data::repositories::StoreRepositoryInMemoryImpl;

#[tokio::main(flavor = "current_thread")]
pub async fn retrieve_available_stores() -> Result<Vec<StoreModel>, String> {
    let store_repository: Box<dyn StoreRepository> =
        Box::new(StoreRepositoryInMemoryImpl::new(HashMap::new()));

    store_repository
        .retrieve_all()
        .await
        .map(|stores| {
            stores
                .iter()
                .map(|store| StoreModel::new(store.id.to_string(), store.name.clone()))
                .collect()
        })
        .map_err(|e| format!("{:?}", e))
}

pub struct StoreModel {
    pub id: String,
    pub name: String,
}

impl StoreModel {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}
