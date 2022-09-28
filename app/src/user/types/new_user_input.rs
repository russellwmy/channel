use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUserInput {
    pub name: String,
    pub image: Option<String>,
}
