use std::io::Error;

use log::info;
use uuid::Uuid;
use web3_anywhere::near::{
    primitives::{query::FunctionArgs, types::AccountId},
    Wallet,
};

use crate::{ config::CHANNEL_CONTRACT_ID};

pub async fn get_groups_debug(wallet: Wallet) -> Result<bool, serde_json::Error> {
    let contract_id = CHANNEL_CONTRACT_ID.parse::<AccountId>().unwrap();

    let bytes = serde_json::to_vec(&{}).unwrap();
    let response = wallet
        .near_rpc_user()
        .view_call(&contract_id, "get_groups_debug", bytes.into())
        .await
        .unwrap();

    serde_json::from_slice(&response.result)
}
