use protocol::{SessionId, UserId};

#[derive(Clone, Debug)]
pub struct SessionState {
    pub session_id: SessionId,
    pub user_id: UserId,
}
