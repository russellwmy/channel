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
    pub creator: AccountId,
    pub uuid: String,
    pub name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GroupUser {
    pub groupUuid: String,
    pub accountId: AccountId,
    pub isAdmin: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    pub accountId: AccountId,
    pub name: String,
    pub data: String,
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
    groupUsers: Vector<GroupUser>,
    users: Vector<User>,
}

impl Default for ChannelContract {
    fn default() -> Self {
        Self {
            groups: Vector::new(b"g"),
            groupUsers: Vector::new(b"j"), //j for "join" table
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
            groupUsers: Vector::new(b"j"), //j for "join" table
            users: Vector::new(b"u"),
        }
    }

    // set group - create / edit group, creator will be joined
    #[payable]
    pub fn set_group(&mut self, uuid: String, name: String) -> bool {
        let sender = env::predecessor_account_id();

        // if group exists edit it
        for i in 0..self.groups.len() {
            let group = self.groups.get(i).unwrap();
            if group.uuid == uuid && group.creator == sender {
                let newGroup = Group {
                    creator: group.creator,
                    uuid: group.uuid,
                    name: name,
                };

                self.groups.replace(i, &newGroup);
                return true;
            }
        }

        // create group
        let group = Group {
            creator: sender.clone(),
            uuid: uuid.clone(),
            name,
        };
        self.groups.push(&group);

        // creator joins group
        let groupUser = GroupUser {
            groupUuid: uuid.clone(),
            accountId: sender.clone(),
            isAdmin: true,
        };
        self.groupUsers.push(&groupUser);

        return true;
    }

    // set user - create / edit user profile
    #[payable]
    pub fn set_user(&mut self, name: String, data: String) -> bool {
        let sender = env::predecessor_account_id();

        // if user exists edit it
        for i in 0..self.users.len() {
            let user = self.users.get(i).unwrap();
            if user.accountId == sender {
                let newUser = User {
                    accountId: sender,
                    name: name,
                    data: data,
                };

                self.users.replace(i, &newUser);
                return true;
            }
        }

        // create user
        let user = User {
            accountId: sender,
            name: name,
            data: data,
        };
        self.users.push(&user);

        return true;
    }

    pub fn get_groups_debug(&self) -> Vec<Group> {
        return self.groups.iter().collect();
    }

    pub fn get_joined_groups(&self, accountId: AccountId) -> Vec<Group> {
        return self
            .groupUsers
            .iter()
            .filter(|gu| gu.accountId == accountId)
            .map(|gu| self.groups.iter().find(|g| g.uuid == gu.groupUuid).unwrap())
            .collect();
    }

    pub fn get_group_users(&self, groupUuid: String) -> Vec<User> {
        return self
            .groupUsers
            .iter()
            .filter(|gu| gu.groupUuid == groupUuid)
            .map(|gu| {
                self.users
                    .iter()
                    .find(|u| u.accountId == gu.accountId)
                    .unwrap()
            })
            .collect();
    }
}
