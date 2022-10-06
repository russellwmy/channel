use dioxus::prelude::*;

use crate::wallet::components::NearConnectButton;

pub fn Header<'a>(cx: Scope<'a>) -> Element {
    let view = rsx! {
        div {
            class: " flex items-center justify-between border-b-2 border-gray-100 p-6 md:space-x-10",
            h1 { "Channel" }
            NearConnectButton{}
        }
    };

    cx.render(view)
}
