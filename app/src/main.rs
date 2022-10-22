#![allow(non_snake_case)]

// #[warn(unused_imports)]
// #[macro_use]
extern crate serde_derive;

use dioxus::{prelude::*, router::Router};

mod app;
mod chatroom;
mod config;
mod home;
mod routes;
mod temp;
mod wallet;

use routes::Routes;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let wallet = use_atom_ref(&cx, crate::wallet::WALLET);

    if !wallet.read().checked {
        wallet.write().check();
    }

    cx.render(rsx!(
        Router {
            div {
            div {
                class: "w-full h-full overflow-y-auto overflow-x-hidden",
                Routes {}
            }
            }
        }
    ))
}
