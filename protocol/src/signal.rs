use crate::{SessionId, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub enum Signal {
    // User signal
    NewUser(UserId),

    // Session signal
    NewSession,
    SessionCreated(SessionId),
    JoinSession(SessionId),
    JoinSessionSuccess(SessionId),
    JoinSessionError(SessionId),

    // Video offer signal
    SdpOffer(SessionId, UserId, String),
    SdpAnswer(SessionId, UserId, String),
    ICECandidate(SessionId, UserId, String),
    ICEError(SessionId, String),

    Debug,
}
