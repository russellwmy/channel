use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetUserInput {
    pub name: String,
    pub image: Option<String>,
}
