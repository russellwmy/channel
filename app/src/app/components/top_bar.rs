use dioxus::prelude::*;

use crate::wallet::{components::NearConnectButton, WALLET};

pub fn TopBar(cx: Scope) -> Element {
    let wallet_state = use_atom_ref(&cx, WALLET);
    let account_id = wallet_state.read().account_id();

    cx.render(rsx! (
        div {
            class: "p-2 flex items-center",
            div {
                class: "flex-1"
            }
            span {
                class: "mr-2",
                match account_id {
                    Some(account_id)=> rsx!("{account_id}"),
                    None => rsx!("")
                }
            }

            NearConnectButton{}
        }

    ))
}
