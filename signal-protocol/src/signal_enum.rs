use serde::{Deserialize, Serialize};
use crate::UserID;
use crate::SessionID;

#[derive(Debug, Serialize, Deserialize)]
pub enum SignalEnum {
    // Return called by the server as soon as the user connects
    NewUser(UserID),

    // To manage a live session between two users
    SessionNew,
    SessionReady(SessionID),
    SessionJoin(SessionID),
    SessionJoinSuccess(SessionID),
    SessionJoinError(SessionID),

    // When Connecting to a Session
    VideoOffer(String, SessionID),
    VideoAnswer(String, SessionID),
    IceCandidate(String, SessionID),
    ICEError(String, SessionID),

    //
    Debug,
}
