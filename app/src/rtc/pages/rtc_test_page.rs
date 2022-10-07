use dioxus::prelude::*;

pub fn RTCTestPage(cx: Scope) -> Element {
    cx.render(rsx!(div {
        class: "w-screen h-screen flex items-center justify-center",
    }))
}
