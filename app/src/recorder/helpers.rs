use js_sys::JsString;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;
use web_sys::Element;
use web_sys::MediaDevices;
use web_sys::MediaStream;
use web_sys::MediaStreamConstraints;
use web_sys::MediaStreamTrack;
use web_sys::Navigator;
use web_sys::Window;


use super::capabilities::DeviceSupportedCapabilities;
use super::error::MediaStreamError;

fn window() -> Result<Window, MediaStreamError> {
    match web_sys::window() {
        Some(win) => Ok(win),
        None => Err(MediaStreamError::StructureError {
            structure: "web_sys Window".to_string(),
            error: "None".to_string(),
        }),
    }
}

pub(crate) fn media_devices(navigator: &Navigator) -> Result<MediaDevices, MediaStreamError> {
    match navigator.media_devices() {
        Ok(media) => Ok(media),
        Err(why) => Err(MediaStreamError::StructureError {
            structure: "MediaDevices".to_string(),
            error: format!("{:?}", why),
        }),
    }
}

fn document(window: &Window) -> Result<Document, MediaStreamError> {
    match window.document() {
        Some(doc) => Ok(doc),
        None => Err(MediaStreamError::StructureError {
            structure: "web_sys Document".to_string(),
            error: "None".to_string(),
        }),
    }
}

fn document_select_elem(doc: &Document, element: &str) -> Result<Element, MediaStreamError> {
    match doc.get_element_by_id(element) {
        Some(elem) => Ok(elem),
        None => {
            return Err(MediaStreamError::StructureError {
                structure: format!("Document {}", element),
                error: "None".to_string(),
            })
        }
    }
}

fn element_cast<T: JsCast, U: JsCast>(from: T, name: &str) -> Result<U, MediaStreamError> {
    if !from.has_type::<U>() {
        return Err(MediaStreamError::StructureError {
            structure: name.to_string(),
            error: "Cannot Cast - No Subtype".to_string(),
        });
    }

    let casted = match from.dyn_into::<U>() {
        Ok(cast) => cast,
        Err(_) => {
            return Err(MediaStreamError::StructureError {
                structure: name.to_string(),
                error: "Casting Error".to_string(),
            });
        }
    };
    Ok(casted)
}

fn element_cast_ref<'a, T: JsCast, U: JsCast>(
    from: &'a T,
    name: &'a str,
) -> Result<&'a U, MediaStreamError> {
    if !from.has_type::<U>() {
        return Err(MediaStreamError::StructureError {
            structure: name.to_string(),
            error: "Cannot Cast - No Subtype".to_string(),
        });
    }

    match from.dyn_ref::<U>() {
        Some(v_e) => Ok(v_e),
        None => Err(MediaStreamError::StructureError {
            structure: name.to_string(),
            error: "Cannot Cast".to_string(),
        }),
    }
}

fn create_element(doc: &Document, element: &str) -> Result<Element, MediaStreamError> {
    match Document::create_element(doc, element) {
        // ???? thank you intellij
        Ok(new_element) => Ok(new_element),
        Err(why) => Err(MediaStreamError::StructureError {
            structure: "Document Video Element".to_string(),
            error: format!("{:?}", why.as_string()),
        }),
    }
}

fn set_autoplay_inline(element: &Element) -> Result<(), MediaStreamError> {
    if let Err(why) = element.set_attribute("autoplay", "autoplay") {
        return Err(MediaStreamError::SetPropertyError {
            property: "Video-autoplay".to_string(),
            value: "autoplay".to_string(),
            error: format!("{:?}", why),
        });
    }

    if let Err(why) = element.set_attribute("playsinline", "playsinline") {
        return Err(MediaStreamError::SetPropertyError {
            property: "Video-playsinline".to_string(),
            value: "playsinline".to_string(),
            error: format!("{:?}", why),
        });
    }

    Ok(())
}

pub(crate) fn query_supported_constraints() -> Result<Vec<DeviceSupportedCapabilities>, MediaStreamError> {
    let window: Window = window().expect("should load window object");
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

pub(crate) async fn request_permission() -> Result<(), MediaStreamError> {
    let window: Window = window()?;
    let navigator = window.navigator();
    let media_devices = media_devices(&navigator)?;

    match media_devices.get_user_media_with_constraints(
        MediaStreamConstraints::new()
            .video(&JsValue::from_bool(true))
            .audio(&JsValue::from_bool(false)),
    ) {
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
