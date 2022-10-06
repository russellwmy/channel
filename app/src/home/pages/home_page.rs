use dioxus::prelude::*;

use crate::{chatroom::components::ChatroomList, home::pages::components::Header};

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! (

        div {
            // class: "mx-auto max-w-7xl px-4 sm:px-6",
            Header{}
            ChatroomList{}
        }
    ))
}
