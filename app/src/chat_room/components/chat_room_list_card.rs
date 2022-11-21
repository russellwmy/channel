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
    let view = rsx! {
        div {
            class: "hover:bg-slate-700",
            button {
                onclick: move |evt| cx.props.onclick.call(evt),
                class: "items-center flex w-full justify-between border-b-2 border-gray-100 p-4 md:space-x-10",
                h4 { "{cx.props.name}"}
                h4 { "Created by {cx.props.creator}"}
            }
        }

    };

    cx.render(view)
}
