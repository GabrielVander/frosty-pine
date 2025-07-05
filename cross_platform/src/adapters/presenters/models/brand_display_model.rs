use expense_tracking::domain::entities::Brand;

pub struct BrandDisplayModel {
    pub name: String,
}

impl From<&Brand> for BrandDisplayModel {
    fn from(value: &Brand) -> Self {
        Self { name: value.name.clone() }
    }
}

impl From<Brand> for BrandDisplayModel {
    fn from(value: Brand) -> Self {
        Self { name: value.name }
    }
}
