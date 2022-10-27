use dioxus::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};

use crate::wallet::components::NearConnectButton;

pub fn HomePage(cx: Scope) -> Element {
    let wallet = use_atom_ref(&cx, crate::wallet::WALLET);

    if !wallet.read().checked {
        wallet.write().check();
    }

    cx.render(rsx! (
        div {
            class: "w-screen h-screen flex items-center justify-center",
            NearConnectButton{}
        }
    ))
}
