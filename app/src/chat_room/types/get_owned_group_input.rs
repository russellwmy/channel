use serde::{Deserialize, Serialize};
use web3_anywhere::near::primitives::types::AccountId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetOwnedGroupsInput {
    pub account_id: AccountId,
}
