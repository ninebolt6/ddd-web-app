use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
}

impl UserEntity {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}
