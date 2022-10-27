use core::str::FromStr;
use std::convert::Infallible;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(inner: String) -> Self {
        UserId(inner)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl FromStr for UserId {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
