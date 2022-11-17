use core::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct RoomId(Uuid);

impl RoomId {
    pub fn new() -> Self {
        RoomId(Uuid::new_v4())
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl FromStr for RoomId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Uuid::from_str(s)?;
        Ok(RoomId(id))
    }
}

impl fmt::Display for RoomId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}