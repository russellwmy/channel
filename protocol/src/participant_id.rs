use core::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, PartialOrd)]
pub struct ParticipantId(Uuid);

impl ParticipantId {
    pub fn new() -> Self {
        ParticipantId(Uuid::now_v1(&[1,1,1,1,1,1]))
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl FromStr for ParticipantId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Uuid::from_str(s)?;
        Ok(ParticipantId(id))
    }
}

impl fmt::Display for ParticipantId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
