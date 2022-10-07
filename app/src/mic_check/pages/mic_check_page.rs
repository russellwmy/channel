use dioxus::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{self, window, HtmlAudioElement, MediaStreamConstraints};

use crate::recorder;

pub fn MicCheckPage(cx: Scope) -> Element {
    let element_id = "mic";
    let recorder_future = use_future(&cx, (), |_| async move {
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&JsValue::from_bool(false));
        constraints.audio(&JsValue::from_bool(true));

        recorder::helpers::request_permission(&constraints)
            .await
            .expect("should get camera permission");

        recorder::Recorder::new(constraints).await
    });

    cx.render(rsx! (
        div {
            class: "w-screen h-screen flex items-center justify-center",
            audio { id: "{element_id}" },
            match recorder_future.value() {
                Some(Ok(result)) => {
                    let element = window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_element_by_id(element_id)
                        .unwrap();
                    let audio_element = element.dyn_into::<HtmlAudioElement>().unwrap();
                    audio_element.set_muted(true);
                    audio_element.set_autoplay(true);
                    audio_element.set_src_object(Some(&result.stream));

                    rsx!("Mircophone ready")
                }
                Some(Err(err)) => rsx!("Error!"),
                None => rsx!("Loading!"),
            }
        }
    ))
}
