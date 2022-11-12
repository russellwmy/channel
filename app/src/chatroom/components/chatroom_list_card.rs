use dioxus::{events::MouseEvent, prelude::*};

#[derive(Props)]
pub struct ChatroomListCardProps<'a> {
    id: &'a str,
    active: bool,
    title: String,
    onclick: EventHandler<'a, MouseEvent>,
}

pub fn ChatroomListCard<'a>(cx: Scope<'a, ChatroomListCardProps<'a>>) -> Element {
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
                h4 { "{cx.props.title}"}
                h4 { "{cx.props.id}"}
            }
        }

    };

    cx.render(view)
}
