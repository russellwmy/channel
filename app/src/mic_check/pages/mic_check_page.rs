use dioxus::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlAudioElement, MediaStreamConstraints};
use webrtc::errors::MediaStreamError;

pub fn MicCheckPage(cx: Scope) -> Element {
    let element_id = "mic";
    let media_check_future = use_future(&cx, (), |_| async move {
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&JsValue::from_bool(false));
        constraints.audio(&JsValue::from_bool(true));

        webrtc::request_permission(&constraints)
            .await
            .map_err(|e| MediaStreamError::InitializeError);

        let stream = webrtc::create_stream(&constraints)
            .await
            .map_err(|e| MediaStreamError::InitializeError);

        stream
    });

    cx.render(rsx! (
        div {
            class: "w-screen h-screen flex items-center justify-center",
            audio { id: "{element_id}" },
            match media_check_future.value() {
                Some(Ok(stream)) => {
                    let element = window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_element_by_id(element_id)
                        .unwrap();
                    let audio_element = element.dyn_into::<HtmlAudioElement>().unwrap();
                    audio_element.set_muted(true);
                    audio_element.set_autoplay(true);
                    audio_element.set_src_object(Some(&stream));

                    rsx!("Mircophone ready")
                }
                Some(Err(err)) => rsx!("Error!"),
                None => rsx!("Loading!"),
            }
        }
    ))
}
