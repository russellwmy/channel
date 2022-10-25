use dioxus::prelude::*;

use crate::{
    chatroom::components::{ChatroomDetail, ChatroomList},
    wallet::components::NearConnectButton
};

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! (

        div {
            // class: "container mx-auto max-w-7xl px-4 sm:px-6",

            div {
                class: "flex flex-col h-screen max-w-7xl",

                div {
                    class: "flex flex-row items-center justify-between border-b-2 border-gray-100 py-2 px-4 md:space-x-10 sticky top-0",
                    h1 { "Channel" }
                    NearConnectButton{}
                }
                div {
                    class: "flex flex-1 flex-row justify-items-center overflow-hidden",
                    div {
                        class: "sm:w-full lg:w-1/4 border-2 overflow-y-scroll",
                        ChatroomList{}
                    }
                    div {
                        class: "sm:w-full lg:w-3/4 flex flex-col border-2 overflow-y-scroll",
                        ChatroomDetail{}
                    }

                }
            }
        }
    ))
}
