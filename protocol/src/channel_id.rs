#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ChannelId(String);

impl ChannelId {
    pub fn new(inner: String) -> Self {
        ChannelId(inner)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<&str> for ChannelId {
    fn from(channel_id: &str) -> Self {
        ChannelId(channel_id.to_string())
    }
}
