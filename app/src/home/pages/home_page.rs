use dioxus::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use webrtc::Session;

use crate::wallet::components::NearConnectButton;

pub fn HomePage(cx: Scope) -> Element {
    let wallet = use_atom_ref(&cx, crate::wallet::WALLET);
    
    use_future(&cx, (), |_| async move {
        let session = Session::new("ws://127.0.0.1:9000").await;
        session
    });

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
