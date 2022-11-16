use dioxus::prelude::*;
use log::info;

use web3_anywhere::near::{
    primitives::types::AccountId,
    Wallet,
};

use crate::{
    config::USER_CONTRACT_ID,
    user::{
        functions::{get_user_info},
        types::{GetUserInfoInput, UserInfoOutput}
    },
    wallet::WALLET,
    
};

pub static USER: AtomRef<UserState> = |_| UserState::new();

pub struct UserState {
    pub registered: bool,
    info: Option<UserInfoOutput>
}

impl UserState {

    pub fn new() -> Self {
        Self {
            registered: false,
            info: Some( UserInfoOutput {
                owner_id: "stephnear.testnet".to_string(),
                name: "steph".to_string(),
                image: None
            } ),
        }
    }

    pub fn info(&self) ->  Option<UserInfoOutput>  {
        self.info.clone()
    }

    pub fn set_info(&mut self, info: UserInfoOutput) {
        self.registered = true;
        self.info = Some(info);
    }

}
