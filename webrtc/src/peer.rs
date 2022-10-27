use protocol::{Signal, UserId};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    BinaryType,
    ErrorEvent,
    MediaStream,
    MediaStreamConstraints,
    RtcPeerConnection,
    WebSocket,
};

use crate::{errors::MediaStreamError, media::create_stream};

#[derive(Clone)]
pub struct Peer {
    pub local_stream: Option<MediaStream>,
    pub user_id: Option<UserId>,
    pub connection: RtcPeerConnection,
}

impl Peer {
    pub fn new() -> Self {
        Self {
            local_stream: None,
            user_id: None,
            connection: RtcPeerConnection::new().unwrap_throw(),
        }
    }

    pub async fn create_audio_stream(&mut self) -> Result<(), MediaStreamError> {
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&JsValue::from_bool(false));
        constraints.audio(&JsValue::from_bool(true));

        let local_stream = create_stream(&constraints).await?;
        self.local_stream = Some(local_stream);

        Ok(())
    }
}
