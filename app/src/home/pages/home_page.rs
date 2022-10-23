use dioxus::prelude::*;

use crate::{
    chatroom::components::{ChatroomDetail, ChatroomList},
    home::pages::components::Header,
};

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! (

        div {
            // class: "mx-auto max-w-7xl px-4 sm:px-6",
    
            div {
                class: "max-w-7xl h-screen",
                Header{}
                div {
                    class: "h-full flex flex-1 flex-row justify-self-center",
                    div {
                        class: "sm:w-full lg:w-1/4 border-2",
                        ChatroomList{}
                    }
                    div {
                        class: "sm:w-full lg:w-3/4 bg-yellow-50",
                        ChatroomDetail{}
                    }
                    
                }
            }
        }
    ))
}

