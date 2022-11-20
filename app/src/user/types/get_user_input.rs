use serde::{Deserialize, Serialize};
use web3_anywhere::near::primitives::types::AccountId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUserInput {
    pub account_id: AccountId,
}
