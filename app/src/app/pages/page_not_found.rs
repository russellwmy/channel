use dioxus::prelude::*;

use crate::app::components::Container;

pub fn PageNotFound(cx: Scope) -> Element {
    cx.render(rsx!(
        Container {
            div {
                class: "md:mt-[30%] text-center",
                h1 {
                    class: "text-4xl md:text-5xl text-white font-bold my-2",
                    "Page not found"
                }
            }
        }
    ))
}
