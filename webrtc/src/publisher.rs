use protocol::UserId;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use web_sys::{MediaStream, MediaStreamConstraints, RtcPeerConnection};

use crate::{errors::MediaStreamError, media::create_stream};

#[derive(Clone)]
pub struct Publisher {
    pub user_id: UserId,
    pub local_stream: Option<MediaStream>,
    pub connection: RtcPeerConnection,
}

impl Publisher {
    pub fn new() -> Self {
        let user_id = UserId::new(Uuid::new_v4().to_string());

        Self {
            local_stream: None,
            user_id,
            connection: RtcPeerConnection::new().unwrap_throw(),
        }
    }

    pub async fn create_stream(&mut self) -> Result<(), MediaStreamError> {
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&JsValue::from_bool(false));
        constraints.audio(&JsValue::from_bool(true));

        let local_stream = create_stream(&constraints).await?;
        self.local_stream = Some(local_stream);

        Ok(())
    }
}
