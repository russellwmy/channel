use js_sys::{JsString, Object};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{MediaDevices, MediaStream, MediaStreamConstraints, MediaStreamTrack, Navigator};

use crate::{device::DeviceSupportedCapabilities, dom::window, errors::MediaStreamError};

pub async fn create_stream(
    constraints: &MediaStreamConstraints,
) -> Result<MediaStream, MediaStreamError> {
    let window = window()?;
    let navigator = window.navigator();
    let media_devices = media_devices(&navigator)?;

    let stream: MediaStream = match media_devices.get_user_media_with_constraints(&constraints) {
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

    Ok(stream)
}

pub fn query_supported_constraints() -> Result<Vec<DeviceSupportedCapabilities>, MediaStreamError> {
    let window = window()?;
    let navigator = window.navigator();
    let media_devices = media_devices(&navigator)?;

    let supported_constraints = JsValue::from(media_devices.get_supported_constraints());
    let dict_supported_constraints = Object::from(supported_constraints);

    let mut capabilities_vec = vec![];
    for constraint in Object::keys(&dict_supported_constraints).iter() {
        let constraint_str = JsValue::from(JsString::from(constraint))
            .as_string()
            .unwrap_or_default();

        // swallow errors
        if let Ok(cap) = DeviceSupportedCapabilities::try_from(constraint_str) {
            capabilities_vec.push(cap);
        }
    }
    Ok(capabilities_vec)
}

pub async fn request_permission(
    constraints: &MediaStreamConstraints,
) -> Result<(), MediaStreamError> {
    let window = window()?;
    let navigator = window.navigator();
    let media_devices = media_devices(&navigator)?;

    match media_devices.get_user_media_with_constraints(constraints) {
        Ok(promise) => {
            let js_future = JsFuture::from(promise);
            match js_future.await {
                Ok(stream) => {
                    let media_stream = MediaStream::from(stream);
                    media_stream
                        .get_tracks()
                        .iter()
                        .for_each(|track| MediaStreamTrack::from(track).stop());
                    Ok(())
                }
                Err(why) => Err(MediaStreamError::OpenStreamError(format!("{:?}", why))),
            }
        }
        Err(why) => Err(MediaStreamError::StructureError {
            structure: "UserMediaPermission".to_string(),
            error: format!("{:?}", why),
        }),
    }
}

pub fn media_devices(navigator: &Navigator) -> Result<MediaDevices, MediaStreamError> {
    match navigator.media_devices() {
        Ok(media) => Ok(media),
        Err(why) => Err(MediaStreamError::StructureError {
            structure: "MediaDevices".to_string(),
            error: format!("{:?}", why),
        }),
    }
}
