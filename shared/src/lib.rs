use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDto {
    pub id: u64,
    pub name: String,
}
