use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::Vector,
    env,
    json_types::U128,
    near_bindgen,
    serde::Serialize,
    AccountId,
    Balance,
};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Group {
    pub id: String,
    pub creator: AccountId,
    pub name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GroupUser {
    pub group_id: String,
    pub account_id: AccountId,
    pub is_admin: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    pub account_id: AccountId,
    pub name: String,
    pub image: Option<String>,
}

// set group - create / edit group, creator will be joined, Done
// delete group

// set user - create / edit user profile, Done
// delete user

// invite to group - admin invites a accountId to a group
// leave/kick group - admin kicks user or user leaves group

// get joined groups - get list of groups joined, Done
// get group users - get user list of a group, Done

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ChannelContract {
    groups: Vector<Group>,
    group_users: Vector<GroupUser>,
    users: Vector<User>,
}

impl Default for ChannelContract {
    fn default() -> Self {
        Self {
            groups: Vector::new(b"g"),
            group_users: Vector::new(b"j"), //j for "join" table
            users: Vector::new(b"u"),
        }
    }
}

#[near_bindgen]
impl ChannelContract {
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        // just clear state on migration
        Self {
            groups: Vector::new(b"g"),
            group_users: Vector::new(b"j"), //j for "join" table
            users: Vector::new(b"u"),
        }
    }

    // set group - create / edit group, creator will be joined
    pub fn set_group(&mut self, id: String, name: String) -> bool {
        let sender = env::predecessor_account_id();

        // if group exists edit it
        for i in 0..self.groups.len() {
            let group = self.groups.get(i).unwrap();
            if group.id == id && group.creator == sender {
                let new_group = Group {
                    creator: group.creator,
                    id: group.id,
                    name: name,
                };

                self.groups.replace(i, &new_group);
                return true;
            }
        }

        // create group
        let group = Group {
            id: id.clone(),
            creator: sender.clone(),
            name,
        };
        self.groups.push(&group);

        // creator joins group
        let group_user = GroupUser {
            group_id: id.clone(),
            account_id: sender.clone(),
            is_admin: true,
        };
        self.group_users.push(&group_user);

        return true;
    }

    // set user - create / edit user profile
    // #[payable]
    pub fn set_user(&mut self, name: String, image: Option<String>) -> bool {
        let sender = env::predecessor_account_id();

        // if user exists edit it
        for i in 0..self.users.len() {
            let user = self.users.get(i).unwrap();
            if user.account_id == sender {
                let new_user = User {
                    account_id: sender,
                    name: name,
                    image: image,
                };

                self.users.replace(i, &new_user);
                return true;
            }
        }

        // create user
        let user = User {
            account_id: sender,
            name: name,
            image: image,
        };
        self.users.push(&user);

        return true;
    }

    pub fn get_groups_debug(&self) -> Vec<Group> {
        return self.groups.iter().collect();
    }

    pub fn get_user(&self, account_id: AccountId) -> Option<User> {
        self.users.iter().find(|u| u.account_id == account_id)
    }

    pub fn get_owned_groups(&self, account_id: AccountId) -> Vec<Group> {
        return self
            .groups
            .iter()
            .filter(|gu| gu.creator == account_id)
            .collect();
    }

    pub fn get_joined_groups(&self, account_id: AccountId) -> Vec<Group> {
        return self
            .group_users
            .iter()
            .filter(|gu| gu.account_id == account_id)
            .map(|gu| self.groups.iter().find(|g| g.id == gu.group_id).unwrap())
            .collect();
    }

    pub fn get_group_users(&self, group_id: String) -> Vec<User> {
        return self
            .group_users
            .iter()
            .filter(|gu| gu.group_id == group_id)
            .map(|gu| {
                self.users
                    .iter()
                    .find(|u| u.account_id == gu.account_id)
                    .unwrap()
            })
            .collect();
    }
}
