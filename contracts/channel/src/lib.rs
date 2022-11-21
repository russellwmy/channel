use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{UnorderedMap, Vector},
    env,
    json_types::U128,
    near_bindgen,
    serde::Serialize,
    AccountId,
    Balance,
};

pub type GroupId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Group {
    pub id: GroupId,
    pub creator: AccountId,
    pub name: String,
    pub users: Vec<GroupUser>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GroupUser {
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
    groups: UnorderedMap<GroupId, Group>,
    users: UnorderedMap<AccountId, User>,
    user_owned_groups: UnorderedMap<AccountId, Vector<GroupId>>,
    // user_joined_groups: UnorderedMap<AccountId, Vector<GroupId>>,
}

impl Default for ChannelContract {
    fn default() -> Self {
        Self {
            groups: UnorderedMap::new(b"g"),
            users: UnorderedMap::new(b"u"),
            user_owned_groups: UnorderedMap::new(b"o"),
        }
    }
}

#[near_bindgen]
impl ChannelContract {
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        // can't reset state, tried clear on the maps, but
        Self {
            groups: UnorderedMap::new(b"g"),
            users: UnorderedMap::new(b"u"),
            user_owned_groups: UnorderedMap::new(b"o"),
        }
    }

    // set group - create / edit group, creator will be joined
    pub fn set_group(&mut self, id: String, name: String) {
        let sender = env::predecessor_account_id();
        let group = self.groups.get(&id);
        let group_data = match group {
            Some(group) => {
                let mut group_mut = group;
                if group_mut.creator == sender {
                    group_mut.name = name;
                }
                group_mut
            }
            None => {
                let group = Group {
                    id: id.clone(),
                    creator: sender.clone(),
                    name,
                    users: vec![GroupUser {
                        account_id: sender.clone(),
                        is_admin: true,
                    }],
                };
                let mut owned_groups = self
                    .user_owned_groups
                    .get(&sender)
                    .unwrap_or(Vector::new(b"o"));
                owned_groups.push(&id);

                self.user_owned_groups.insert(&sender, &owned_groups);

                group
            }
        };

        self.groups.insert(&group_data.id, &group_data);
    }

    // set user - create / edit user profile
    pub fn set_user(&mut self, name: String, image: Option<String>) {
        let sender = env::predecessor_account_id();
        let user = self.users.get(&sender);
        let user_data = match user {
            Some(user) => {
                let mut user_mut = user;
                user_mut.name = name;
                user_mut.image = image;

                user_mut
            }
            None => User {
                account_id: sender,
                name: name,
                image: image,
            },
        };

        self.users.insert(&user_data.account_id, &user_data);
    }

    // pub fn get_groups_debug(&self) -> Vec<Group> {
    //     return self.groups.iter().collect();
    // }

    pub fn get_group(&self, group_id: GroupId) -> Option<Group> {
        self.groups.get(&group_id)
    }

    pub fn get_user(&self, account_id: AccountId) -> Option<User> {
        self.users.get(&account_id)
    }

    pub fn get_owned_groups(&self, account_id: AccountId) -> Vec<Group> {
        let user_owned_groups = self.user_owned_groups.get(&account_id);
        match user_owned_groups {
            Some(user_owned_groups) => user_owned_groups
                .iter()
                .map(|g| self.groups.get(&g).unwrap())
                .collect(),
            Non => vec![],
        }
    }

    pub fn join_group(&mut self, id: String) {
        let sender = env::predecessor_account_id();

        let mut group = self.groups.get(&id).unwrap();

        // if already joined, return
        for i in 0..group.users.len() {
            let groupUser = group.users.get(i).unwrap();
            if groupUser.account_id == sender {
                return;
            }
        }

        let groupUser = GroupUser {
            account_id: sender,
            is_admin: false,
        };
        group.users.push(groupUser);

        self.groups.insert(&group.id, &group);
    }

    // pub fn get_joined_groups(&self, account_id: AccountId) -> Vec<Group> {
    //     return self
    //         .group_users
    //         .iter()
    //         .filter(|gu| gu.account_id == account_id)
    //         .map(|gu| self.groups.iter().find(|g| g.id == gu.group_id).unwrap())
    //         .collect();
    // }

    // pub fn get_group_users(&self, group_id: String) -> Vec<User> {
    //     return self
    //         .group_users
    //         .iter()
    //         .filter(|gu| gu.group_id == group_id)
    //         .map(|gu| {
    //             self.users
    //                 .iter()
    //                 .find(|u| u.account_id == gu.account_id)
    //                 .unwrap()
    //         })
    //         .collect();
    // }
}
