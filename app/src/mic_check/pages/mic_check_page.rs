use dioxus::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{self, window, HtmlAudioElement, MediaStreamConstraints};

use crate::recorder::{self, meter};

pub fn MicCheckPage(cx: Scope) -> Element {
    let element_id = "mic";
    let recorder = use_future(&cx, (), |_| async move {
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&JsValue::from_bool(false));
        constraints.audio(&JsValue::from_bool(true));

        recorder::helpers::request_permission(&constraints)
            .await
            .expect("should get camera permission");

        recorder::Recorder::new(constraints).await
    });

    let status = match recorder.value() {
        Some(Ok(recorder)) => {
            let element = window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id(element_id)
                .unwrap();
            let audio_element = element.dyn_into::<HtmlAudioElement>().unwrap();
            audio_element.set_src_object(Some(&recorder.stream));

            recorder.volume();
            ""
        }
        Some(Err(err)) => "error!",
        None => "loading!",
    };

    cx.render(rsx! (
        div {
            class: "w-screen h-screen flex items-center justify-center",
            "Checking Mic",
            audio {
                id: "{element_id}",
                autoplay: "true",
            }
        }
    ))
}
