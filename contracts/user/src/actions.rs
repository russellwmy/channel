use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn create_user(&mut self, data: NewUserInput) {
        // Check inputs
        let caller_id: AccountId = env::predecessor_account_id();
        assert!(
            !self.users.get(&caller_id.clone()).is_some(),
            "user exists."
        );
        assert!(data.name.len() != 0, "missing name.");
        assert!(data.name.len() <= 255, "name is too long.");
        assert!(data.image.len() != 0, "missing image.");
        assert!(data.image.len() <= 2048, "image is too long.");

        let mut admin_ids = HashSet::new();
        admin_ids.insert(caller_id.clone());

        let user_info = UserInfo {
            owner_id: caller_id.clone(),
            name: data.name.clone(),
            image: data.image.clone(),
        };

        self.users.insert(&caller_id, &user_info);
    }
}
