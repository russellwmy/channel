use web3_anywhere::near::{
    primitives::types::AccountId
};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GROUP {
    pub creator: AccountId,
    pub uuid: String,
    pub name: String,
}