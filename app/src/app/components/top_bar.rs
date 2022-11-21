use dioxus::prelude::*;

use crate::{
    user::components::UserStateChecker,
    wallet::{components::NearConnectButton, WALLET},
};

pub fn TopBar(cx: Scope) -> Element {
    let wallet_state = use_atom_ref(&cx, WALLET);
    let account_id = wallet_state.read().account_id();

    cx.render(rsx! (
        div {
            class: "p-2 flex items-center",
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
