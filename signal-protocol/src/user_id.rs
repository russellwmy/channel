use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct UserID(String);

impl UserID {
    pub fn new(inner: String) -> Self {
        UserID(inner)
    }
    pub fn inner(self) -> String {
        self.0
    }
}
