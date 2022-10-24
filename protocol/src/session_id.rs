#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct SessionId(String);

impl SessionId {
    pub fn new(inner: String) -> Self {
        SessionId(inner)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<&str> for SessionId {
    fn from(session_id: &str) -> Self {
        SessionId(session_id.to_string())
    }
}
