use std::io::Error;

use web3_anywhere::near::{
    primitives::{query::FunctionArgs, types::AccountId},
    Wallet,
};

use crate::{
    config::USER_CONTRACT_ID,
    user::types::{GetUserInfoInput, UserInfoOutput},
};

pub async fn get_user_info(
    wallet: Wallet,
    input: GetUserInfoInput,
) -> Result<UserInfoOutput, serde_json::Error> {
    let contract_id = USER_CONTRACT_ID.parse::<AccountId>().unwrap();
    let bytes = serde_json::to_vec(&input).unwrap();
    let response = wallet
        .near_rpc_user()
        .view_call(&contract_id, "get_user_info", bytes.into())
        .await
        .unwrap();
    serde_json::from_slice(&response.result)
}
