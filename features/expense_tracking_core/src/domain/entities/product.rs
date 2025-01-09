use crate::domain::entities::Brand;
use crate::domain::entities::Category;

#[derive(Debug, Clone, PartialEq)]
pub struct Product<'a> {
    name: String,
    brand: &'a Brand,
    category: &'a Category,
}

impl<'a> Product<'a> {
    pub fn new(name: String, brand: &'a Brand, category: &'a Category) -> Self {
        Self {
            name,
            brand,
            category,
        }
    }
}
