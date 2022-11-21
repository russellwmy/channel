use web3_anywhere::near::primitives::types::AccountId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupOutput {
    pub creator: AccountId,
    pub id: String,
    pub name: String,
}
