use dioxus::prelude::*;

use crate::{
    app::components::{Container, TopBar}, chat_room::components::ChatRoomList,
};

pub fn LobbyPage(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "w-screen h-screen",
            TopBar {}
            Container {
                ChatRoomList{}
            }
        }
    ))
}
