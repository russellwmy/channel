use crate::{UserId, ChannelId};

#[derive(Debug, Serialize, Deserialize)]
pub enum Signal {
    // User signal
    NewUser(UserId),

    // Session signal
    NewChannel,
    ChannelCreated(ChannelId),
    JoinChannel(ChannelId),
    JoinChannelSuccess(ChannelId),
    JoinChannelError(ChannelId),

    // Video offer signal
    SdpOffer(ChannelId, UserId, String),
    SdpAnswer(ChannelId, UserId, String),
    ICECandidate(ChannelId, UserId, String),
    ICEError(ChannelId, String),

    Debug,
}
