use dioxus::prelude::*;
use log::info;
use web3_anywhere::near::{primitives::types::AccountId, Wallet};

use crate::{
    config::CONTRACT_ID,
    user::{
        functions::get_user,
        types::{GetUserInput, UserOutput},
    },
    wallet::WALLET,
};

pub static USER: AtomRef<UserState> = |_| UserState::new();

pub struct UserState {
    pub registered: bool,
    user: Option<UserOutput>,
}

impl UserState {
    pub fn new() -> Self {
        Self {
            registered: false,
            user: None,
        }
    }

    pub fn user(&self) -> Option<UserOutput> {
        self.user.clone()
    }

    pub fn set_info(&mut self, user: UserOutput) {
        self.registered = true;
        self.user = Some(user);
    }
}
