use dioxus::prelude::*;

mod config;
mod user;
mod wallet;

use wallet::components::NearConnectButton;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
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
