#[derive(Debug, Clone, Default, PartialEq)]
pub struct Category {
    name: String,
}

impl Category {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
