use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserOutput {
    pub account_id: String,
    pub name: String,
    pub image: Option<String>,
}
