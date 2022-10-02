use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::MediaStreamConstraints;
use web_sys::MediaStream;
use web_sys::MediaStreamTrack;
use web_sys::Navigator;
use web_sys::window;
use web_sys::Window;
use web_sys::MediaDevices;

use super::error::MediaStreamError;
use super::helpers::media_devices;

pub struct Recorder {
    pub media_stream: MediaStream,
    pub constraints: MediaStreamConstraints,
}

impl Recorder {
    pub async fn new(constraints: MediaStreamConstraints) -> Result<Self, MediaStreamError> {
        let window: Window = window().expect("should load window object");
        let navigator = window.navigator();
        let media_devices = media_devices(&navigator)?;

        let stream: MediaStream = match media_devices.get_user_media_with_constraints(&constraints)
        {
            Ok(promise) => {
                let future = JsFuture::from(promise);
                match future.await {
                    Ok(stream) => {
                        let media_stream: MediaStream = MediaStream::from(stream);
                        media_stream
                    }
                    Err(why) => {
                        return Err(MediaStreamError::StructureError {
                            structure: "MediaDevicesGetUserMediaJsFuture".to_string(),
                            error: format!("{:?}", why),
                        })
                    }
                }
            }
            Err(why) => {
                return Err(MediaStreamError::StructureError {
                    structure: "MediaDevicesGetUserMedia".to_string(),
                    error: format!("{:?}", why),
                })
            }
        };

        Ok(Self {
            media_stream: stream,
            constraints,
        })
    }
}