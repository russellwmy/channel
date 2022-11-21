use dioxus::prelude::*;

use crate::chat_room::components::{ChatRoomDetail, ChatRoomList};

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! (

        div {
            div {
                class: "flex flex-col h-screen max-w-7xl",

                div {
                    class: "flex flex-1 flex-row justify-items-center overflow-hidden",
                    div {
                        class: "min-w-[280px] border-b-2 border-x-2 overflow-y-scroll",
                        ChatRoomList{}
                    }
                    div {
                        class: "max-w-[1000px] flex flex-1 flex-col border-r-2 border-b-2 overflow-y-scroll",
                        ChatRoomDetail{}
                    }

                }
            }
        }
    ))
}

// sm:w-full lg:w-3/4
