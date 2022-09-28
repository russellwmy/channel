use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfoOutput {
    pub owner_id: String,
    pub name: String,
    pub image: Option<String>,
}
