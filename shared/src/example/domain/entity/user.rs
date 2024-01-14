use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserEntity {
    pub id: Uuid,
    pub name: String,
}

impl UserEntity {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
        }
    }

    pub fn from(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}
