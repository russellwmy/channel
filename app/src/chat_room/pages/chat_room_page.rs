use dioxus::prelude::*;

use crate::{
    app::components::{Container, TopBar},
    chat_room::components::ChatRoom,
};

pub fn ChatRoomPage(cx: Scope) -> Element {
    let room_id = use_route(&cx).last_segment()?;

    cx.render(rsx! (
        div {
            class: "w-screen h-screen",
            TopBar {}
            Container {
                ChatRoom{
                    room_id: room_id.to_string()
                }
            }
        }
    ))
}
