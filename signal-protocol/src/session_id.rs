use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct SessionID(String);

impl SessionID {
    pub fn new(inner: String) -> Self {
        SessionID(inner)
    }
    pub fn inner(self) -> String {
        self.0
    }
}

impl From<&str> for SessionID {
    fn from(session_id: &str) -> Self {
        SessionID(session_id.to_string())
    }
}
