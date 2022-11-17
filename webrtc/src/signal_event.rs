#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum SignalEvent {
    NewUser,
    NewChannel,
    ChannelCreated,
    JoinChannelSuccess,
    JoinChannelError,
    SdpAnswer,
    ICECandidate,
    ICEError,
}
