use crate::{ParticipantId, RoomId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Signal {
    ParticipantRegistered(ParticipantId),
    
    NewParticipantJoined(RoomId, ParticipantId),
    ParticipantLeft(RoomId, ParticipantId),
    JoinRoom(RoomId),
    JoinRoomSuccess(RoomId),
    JoinRoomError(RoomId),
    LeaveRoom(RoomId),
    PeerLeft(RoomId, ParticipantId),

    // Video offer signal
    SdpOffer(RoomId, ParticipantId, String),
    SdpAnswer(RoomId, ParticipantId, String),
    ICECandidate(RoomId, ParticipantId, String),
    ICEError(RoomId, String),

    Debug,
}
