use web3_anywhere::near::{primitives::types::AccountId, Wallet};

use crate::{
    chat_room::types::{GetOwnedGroupsInput, GroupOutput},
    config::CONTRACT_ID,
    errors::ContractCallError,
};

pub async fn get_owned_groups(
    wallet: Wallet,
    input: GetOwnedGroupsInput,
) -> Result<Vec<GroupOutput>, ContractCallError> {
    let contract_id = CONTRACT_ID.parse::<AccountId>().unwrap();
    let bytes = serde_json::to_vec(&input).unwrap();
    let response = wallet
        .near_rpc_user()
        .view_call(&contract_id, "get_owned_groups", bytes.into())
        .await
        .map_err(|_| ContractCallError::CallFail("no result".to_string()))?;
    serde_json::from_slice(&response.result).map_err(|_| ContractCallError::ResultParseFail)
}
