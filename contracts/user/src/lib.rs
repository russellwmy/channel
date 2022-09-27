use std::collections::HashSet;

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupSet, UnorderedMap},
    env,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
};

mod actions;
mod inputs;
mod models;
mod views;

use crate::{actions::*, inputs::*, models::*, views::*};

const CONTRACT_OWNER_ID: &str = "user.channel.testnet";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub admin_ids: LookupSet<AccountId>,
    pub users: UnorderedMap<AccountId, UserInfo>,
}

impl Default for Contract {
    fn default() -> Self {
        let owner_id = CONTRACT_OWNER_ID.parse::<AccountId>().unwrap();
        let mut admin_ids = LookupSet::new(b"u");
        admin_ids.insert(&owner_id);
        let users = UnorderedMap::new(b"d");

        Self {
            owner_id,
            admin_ids,
            users,
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let mut admin_ids = LookupSet::new(b"u");
        admin_ids.insert(&owner_id);
        let users = UnorderedMap::new(b"d");
        Self {
            owner_id,
            admin_ids,
            users,
        }
    }
}
