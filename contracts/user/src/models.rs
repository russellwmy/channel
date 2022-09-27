use crate::*;

#[derive(Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserInfo {
    pub owner_id: AccountId,
    pub name: String,
    pub image: String,
}
