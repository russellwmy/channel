use dioxus::prelude::*;

#[derive(Props)]
pub struct IconProps<'a> {
    name: &'a str,
    color: String,
    size: String,
    bg_color: String,
}

pub fn Icon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let view = rsx! {
        div {
            class: "flex justify-center {cx.props.size} rounded-full bg-neutral-100 {cx.props.bg_color}" ,
            i {
                class: "{cx.props.name} self-center {cx.props.color}"
            }
        }
    };

    cx.render(view)
}
