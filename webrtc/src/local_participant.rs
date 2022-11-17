use protocol::ParticipantId;
use web_sys::MediaStream;

#[derive(Debug, Clone)]
pub struct LocalParticipant {
    id: ParticipantId,
    streams: Vec<MediaStream>,
}

impl LocalParticipant {
    pub fn new_with_id(id: ParticipantId) -> Self {
        Self {
            id,
            streams: vec![],
        }
    }

    pub fn id(&self) -> ParticipantId {
        self.id.clone()
    }
}
