use expense_tracking::domain::entities::Brand;

pub struct BrandModel {
    pub name: String,
}

impl From<Brand> for BrandModel {
    fn from(value: Brand) -> Self {
        Self { name: value.name }
    }
}
