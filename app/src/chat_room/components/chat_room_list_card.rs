use dioxus::{events::MouseEvent, prelude::*};

#[derive(Props)]
pub struct ChatRoomListCardProps<'a> {
    id: &'a str,
    active: bool,
    name: String,
    creator: String,
    onclick: EventHandler<'a, MouseEvent>,
}

pub fn ChatRoomListCard<'a>(cx: Scope<'a, ChatRoomListCardProps<'a>>) -> Element {
    let bg_color = if cx.props.active {
        "bg-slate-50"
    } else {
        "bg-transparent"
    };
    let view = rsx! {
        div {
            // key: cx.props.id,
            button {
                onclick: move |evt| cx.props.onclick.call(evt),
                class: "items-center flex w-full justify-between border-b-2 border-gray-100 p-4 md:space-x-10 {bg_color} ",
                h4 { "{cx.props.name}"}
                h4 { "Created by {cx.props.creator}"}
            }
        }

    };

    cx.render(view)
}
