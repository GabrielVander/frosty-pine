use uuid::Uuid;
use uuid_b64::UuidB64;

use crate::domain::entities::Brand;
use crate::domain::entities::Category;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Product {
    pub id: UuidB64,
    pub name: String,
    pub brand: Brand,
    pub category: Category,
}

impl Product {
    pub fn new(id: Option<UuidB64>, name: String, brand: Brand, category: Category) -> Self {
        Self {
            id: id.unwrap_or_else(|| UuidB64::from(Uuid::new_v4())),
            name,
            brand,
            category,
        }
    }
}
