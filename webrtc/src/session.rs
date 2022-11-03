use protocol::SessionId;
use uuid::Uuid;
use web_sys::WebSocket;

use crate::{
    errors::SessionError,
    publisher::Publisher,
    signalling::{connect_server, create_session, join_session},
};

#[derive(Clone, Debug)]
pub struct Session {
    session_id: SessionId,
    signal_server: WebSocket,
}

impl Session {
    pub async fn new(session_id: SessionId) -> Result<Self, SessionError> {
        let signal_server = connect_server(url).await;

        Ok(Self {
            session_id,
            signal_server,
        })
    }
}
