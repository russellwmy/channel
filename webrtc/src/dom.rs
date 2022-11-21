use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlAudioElement, HtmlVideoElement, MediaStream, Window};

use crate::errors::MediaStreamError;

pub fn window() -> Result<Window, MediaStreamError> {
    match web_sys::window() {
        Some(win) => Ok(win),
        None => Err(MediaStreamError::StructureError {
            structure: "web_sys Window".to_string(),
            error: "None".to_string(),
        }),
    }
}

pub fn document() -> Result<Document, MediaStreamError> {
    let window = window()?;
    let document = window.document().unwrap();

    Ok(document)
}

pub fn attach_stream(stream: MediaStream) {
    if let Ok(document) = document() {
        log::info!("attach_stream");
        let player_node = document
            .create_element("audio")
            .unwrap()
            .dyn_into::<HtmlAudioElement>()
            .unwrap();
        player_node.set_muted(false);
        player_node.set_autoplay(true);

        player_node.set_src_object(Some(&stream));

        let body = document
            .get_elements_by_tag_name("body")
            .get_with_index(0)
            .unwrap();
        let _ = body.append_child(&player_node);
    }
}
