#[derive(Debug, Clone, Default, PartialEq)]
pub struct Store {
    name: String,
}

impl Store {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
