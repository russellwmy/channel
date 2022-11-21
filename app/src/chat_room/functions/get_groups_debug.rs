use std::{io::Error, vec::Vec};

use log::info;
use uuid::Uuid;
use web3_anywhere::near::{
    primitives::{query::FunctionArgs, types::AccountId},
    Wallet,
};

<<<<<<< HEAD:app/src/chat_room/functions/get_groups_debug.rs
use crate::{ config::CHANNEL_CONTRACT_ID};
=======
use crate::{
    chatroom::types::{Group, SetGroupInput},
    config::CHANNEL_CONTRACT_ID,
};
>>>>>>> main:app/src/chatroom/functions/get_groups_debug.rs

pub async fn get_groups_debug(wallet: Wallet) -> Result<Vec<Group>, serde_json::Error> {
    let contract_id = CHANNEL_CONTRACT_ID.parse::<AccountId>().unwrap();

    let bytes = serde_json::to_vec(&{}).unwrap();
    let response = wallet
        .near_rpc_user()
        .view_call(&contract_id, "get_groups_debug", bytes.into())
        .await
        .unwrap();

    serde_json::from_slice(&response.result)
}
