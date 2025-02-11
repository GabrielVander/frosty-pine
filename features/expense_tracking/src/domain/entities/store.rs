use uuid::Uuid;
use uuid_b64::UuidB64;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Store {
    id: UuidB64,
    name: String,
}

impl Store {
    pub fn new(id: Option<UuidB64>, name: String) -> Self {
        Self {
            id: id.unwrap_or_else(|| UuidB64::from(Uuid::new_v4())),
            name,
        }
    }
}
