use dioxus::{events::MouseEvent, prelude::*};

#[derive(Props)]
pub struct ConnectButtonProps<'a> {
    #[props(default)]
    connected: bool,
    onclick: EventHandler<'a, MouseEvent>,
}

pub fn ConnectButton<'a>(cx: Scope<'a, ConnectButtonProps<'a>>) -> Element {
    let view = match cx.props.connected {
        true => {
            rsx!(
                button {
                    onclick: move |evt| cx.props.onclick.call(evt),
                    class: "btn md:btn-circle",
                    span {
                        class: "mr-2 font-bold",
                        "Disconnect"
                    }
                    i {
                        class: "fa-solid fa-arrow-right-from-bracket"
                    }
                }
            )
        }
        false => rsx!(
            button {
                onclick: move |evt| cx.props.onclick.call(evt),
                class: "btn primary normal-case",
                i {
                    class: "fa-solid fa-wallet"
                }
                span {
                    class: "ml-2 font-bold",
                    "Connect"
                }
            }
        ),
    };

    cx.render(view)
}
