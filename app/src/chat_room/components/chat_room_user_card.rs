use dioxus::prelude::*;

use crate::{components::Icon, constant::COLOR_LIST};

#[derive(PartialEq, Props)]
pub struct ChatRoomUserCardProps {
    address: String,
    muted: bool,
}

pub fn ChatRoomUserCard(cx: Scope<ChatRoomUserCardProps>) -> Element {
    let bg_color = COLOR_LIST[2];
    let icon_status = if cx.props.muted {
        "fa-solid fa-microphone-slash"
    } else {
        "fa-solid fa-microphone "
    };
    let view = rsx! {
        div {
            class: "flex flex-col justify-end w-3/4 aspect-video rounded-2xl p-2 m-2 {bg_color} ",
            div {
                class: "flex justify-between",
                div {
                    class: "rounded-2xl py-2 px-4 bg-neutral-100 text-sm",
                    p {
                        "{cx.props.address}"
                    }

                }
                Icon{
                    name: icon_status,
                    color: "text-black".to_owned(),
                    size: "w-10 h-10".to_owned(),
                    bg_color: "bg-neutral-100".to_owned(),
                }
            }
        }
    };

    cx.render(view)
}
