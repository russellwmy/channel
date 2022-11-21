use dioxus::prelude::*;

use crate::{
    user::components::UserStateChecker,
    wallet::{components::NearConnectButton, WALLET},
};

pub fn TopBar(cx: Scope) -> Element {
    let router = use_router(&cx);
    let wallet_state = use_atom_ref(&cx, WALLET);
    let account_id = wallet_state.read().account_id();

    cx.render(rsx! (
        div {
            class: "p-2 flex items-center",
            div {
                button {
                    class: "btn btn-rounded",
                    onclick: move |_|{
                        router.push_route("/", None, None);
                    },
                    i {
                        class: "fa-solid fa-home"
                    }
                    span {
                        class: "ml-2",
                        "Home"
                    }
                }
            }
            div {
                class: "flex-1"

            }
            UserStateChecker{}
            match account_id {
                Some(account_id)=> rsx!( span {
                    class: "mr-2",
                    "{account_id}"
                }),
                None => rsx!("")
            }

            NearConnectButton{}
        }

    ))
}
