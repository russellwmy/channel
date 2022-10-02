use dioxus::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::MediaStreamConstraints;

use crate::recorder;

pub fn MicCheckPage(cx: Scope) -> Element {
    let recorder = use_future(&cx, (), |_| async move {
        recorder::helpers::request_permission()
            .await
            .expect("should get camera permission");
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&JsValue::from_bool(false));
        constraints.audio(&JsValue::from_bool(true));

        recorder::Recorder::new(constraints).await
    });

    let status = match recorder.value() {
        Some(Ok(val)) => "ready!",
        Some(Err(err)) => "errored!",
        None => "loading!",
    };
    cx.render(rsx! (
        div {
            class: "w-screen h-screen flex items-center justify-center",
            "Checking Mic"
        }
    ))
}
