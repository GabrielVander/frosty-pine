use uuid::Uuid;
use uuid_b64::UuidB64;

use crate::domain::entities::Brand;
use crate::domain::entities::Category;

#[derive(Debug, Clone, PartialEq)]
pub struct Product<'a> {
    id: UuidB64,
    name: String,
    brand: &'a Brand,
    category: &'a Category,
}

impl<'a> Product<'a> {
    pub fn new(
        id: Option<UuidB64>,
        name: String,
        brand: &'a Brand,
        category: &'a Category,
    ) -> Self {
        Self {
            id: id.unwrap_or_else(|| UuidB64::from(Uuid::new_v4())),
            name,
            brand,
            category,
        }
    }
}
