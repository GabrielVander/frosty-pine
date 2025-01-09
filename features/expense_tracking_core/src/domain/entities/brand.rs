#[derive(Debug, Clone, Default, PartialEq)]
pub struct Brand {
    name: String,
}

impl Brand {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
