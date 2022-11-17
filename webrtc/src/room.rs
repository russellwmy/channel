use std::collections::HashMap;

use protocol::{ParticipantId, RoomId};

use crate::{local_participant::LocalParticipant, participant::Participant};

#[derive(Debug, Clone)]
pub struct Room {
    id: Option<RoomId>,
    local_participant: Option<LocalParticipant>,
    participants: HashMap<ParticipantId, Participant>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            id: None,
            local_participant: None,
            participants: HashMap::new(),
        }
    }

    pub fn set_local_participant(&mut self, local_participant: LocalParticipant) {
        self.local_participant = Some(local_participant);
    }

    pub fn local_participant(&self) -> Option<LocalParticipant> {
        self.local_participant.clone()
    }

    pub fn set_id(&mut self, id: RoomId) {
        self.id = Some(id);
    }

    pub fn id(&self) -> Option<RoomId> {
        self.id.clone()
    }

    pub fn participants(&self) -> Vec<Participant> {
        self.participants
            .values()
            .map(|x| x.clone())
            .collect::<Vec<Participant>>()
    }
    pub fn get_participant(&self, participant_id: ParticipantId) -> Option<Participant> {
        match self.participants.get(&participant_id) {
            Some(participant) => Some(participant.clone()),
            None => None,
        }
    }

    pub fn add_participant(&mut self, participant: Participant) {
        self.participants
            .insert(participant.id(), participant.clone());
    }

    pub fn remove_participant(&mut self, participant: Participant) {
        self.participants.remove(&participant.id());
    }
}
