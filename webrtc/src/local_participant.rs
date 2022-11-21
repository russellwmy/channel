use protocol::ParticipantId;
use wasm_bindgen::JsCast;
use web_sys::{MediaStream, MediaStreamTrack};

use crate::{errors::MediaStreamError, media};

#[derive(Debug, Clone)]
pub struct LocalParticipant {
    id: ParticipantId,
    stream: MediaStream,
    is_muted: bool,
}

impl LocalParticipant {
    pub fn new_with_id(id: ParticipantId) -> Self {
        Self { id, stream: MediaStream::new().unwrap(), is_muted: true }
    }

    pub fn id(&self) -> ParticipantId {
        self.id.clone()
    }

    pub async fn init_streaming(&mut self) -> Result<(), MediaStreamError> {
        let constraints = media::get_contraints(false, true);
        let stream = media::create_stream(&constraints).await?;
        for track in stream.get_tracks().iter() {
            let track = track.dyn_into::<MediaStreamTrack>().unwrap();
            track.set_enabled(false);
            self.stream.add_track(&track);
        }
        Ok(())
    }

    pub async fn start_streaming(&mut self) -> Result<(), MediaStreamError> {
        for track in self.stream.get_tracks().iter() {
            let track = track.dyn_into::<MediaStreamTrack>().unwrap();
            track.set_enabled(true);
        }
        self.is_muted = false;
        Ok(())
    }

    pub async fn stop_streaming(&mut self) -> Result<(), MediaStreamError> {
        for track in self.stream.get_tracks().iter() {
            let track = track.dyn_into::<MediaStreamTrack>().unwrap();
            track.set_enabled(false);
        }
        self.is_muted = true;
        Ok(())
    }

    pub fn muted(&self) ->bool {
        self.is_muted
    }
    
    pub fn stream(&self) -> &MediaStream {
        &self.stream
    }
}
