use crate::*;

#[near_bindgen]
impl Contract {
    pub fn get_user_info(&self, account_id: AccountId) -> UserInfo {
        assert!(self.users.get(&account_id).is_some(), "no user info.");
        self.users.get(&account_id).unwrap()
    }
}
