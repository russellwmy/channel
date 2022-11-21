use dioxus::prelude::*;

use crate::{components::Icon, constant::COLOR_LIST};

#[derive(PartialEq, Props)]
pub struct ChatRoomUserCardProps {
    account_id: String,
    is_admin: bool,
    muted: bool,
}

pub fn ChatRoomUserCard(cx: Scope<ChatRoomUserCardProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "flex justify-between rounded-2xl bg-neutral-100 px-4 items-center",
            div {
                class: "flex items-center text-slate-800",
                p {
                    class: "mr-2",
                    "{cx.props.account_id}"
                }
                match cx.props.is_admin {
                    true => rsx!(i { class: "fa-solid fa-star"}),
                    false => rsx!("")
                }
            }
            Icon{
                name: match cx.props.muted {
                    true => "fa-solid fa-microphone-slash",
                    false => "fa-solid fa-microphone "
                },
                color: "text-black".to_owned(),
                size: "w-10 h-10".to_owned(),
                bg_color: "bg-neutral-100".to_owned(),
            }
        }

    })
}
