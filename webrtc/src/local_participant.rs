use protocol::ParticipantId;
use wasm_bindgen::JsCast;
use web_sys::{MediaStream, MediaStreamTrack};

use crate::{errors::MediaStreamError, media};

#[derive(Debug, Clone)]
pub struct LocalParticipant {
    id: ParticipantId,
    stream: Option<MediaStream>,
}

impl LocalParticipant {
    pub fn new_with_id(id: ParticipantId) -> Self {
        Self { id, stream: None }
    }

    pub fn id(&self) -> ParticipantId {
        self.id.clone()
    }

    pub async fn start_stream(&mut self) -> Result<(), MediaStreamError> {
        let constraints = media::get_contraints(false, true);
        let stream = media::create_stream(&constraints).await?;
        for track in stream.get_tracks().iter() {
            let track = track.dyn_into::<MediaStreamTrack>().unwrap();
            // track.set_enabled(false);
        }
        self.stream = Some(stream);
        Ok(())
    }

    pub async fn stop_stream(&mut self) -> Result<(), MediaStreamError> {
        log::info!("stop stream: {:?}", self.stream.is_some());
        match self.stream.clone() {
            Some(stream) => {
                for track in stream.get_tracks().iter() {
                    let track = track.dyn_into::<MediaStreamTrack>().unwrap();
                    track.stop();
                }
                log::info!("stop all tracks");
            }
            _ => {}
        };

        Ok(())
    }

    pub fn stream(&self) -> Option<MediaStream> {
        self.stream.clone()
    }
}
