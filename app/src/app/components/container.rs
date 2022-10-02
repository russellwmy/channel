use dioxus::prelude::*;

#[derive(Props)]
pub struct ContainerProps<'a> {
    #[props(optional)]
    class: Option<&'a str>,
    children: Element<'a>,
}

pub fn Container<'a>(cx: Scope<'a, ContainerProps<'a>>) -> Element {
    let custom_class = cx.props.class.unwrap_or("");
    let class = vec!["w-full xl:w-[1024px] h-fit mx-auto p-2", &custom_class].join(" ");

    cx.render(rsx!(
        div {
            class: "{class}",
            &cx.props.children
        }
    ))
}
