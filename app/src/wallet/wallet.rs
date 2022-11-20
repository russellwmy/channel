use dioxus::prelude::AtomRef;
use web3_anywhere::{
    crypto::PublicKey,
    near::{primitives::types::AccountId, Wallet},
};

use crate::wallet::utils;

pub static WALLET: AtomRef<WalletState> = |_| WalletState::new();

pub struct WalletState {
    pub checked: bool,
    wallet: Wallet,
}

impl WalletState {
    pub fn new() -> Self {
        let wallet = utils::connect_wallet();
        Self {
            wallet,
            checked: false,
        }
    }

    pub fn set_wallet(&mut self, wallet: Wallet) {
        self.wallet = wallet;
    }

    pub fn wallet(&self) -> Wallet {
        self.wallet.clone()
    }

    pub fn is_connected(&self) -> bool {
        self.wallet.is_connected()
    }

    pub fn public_key(&self) -> Option<PublicKey> {
        self.wallet.public_key()
    }

    pub fn account_id(&self) -> Option<AccountId> {
        self.wallet.account_id()
    }

    pub fn check(&mut self) {
        self.wallet.complete_sign_in_with_access_key();
        self.checked = true;
    }

    pub async fn connect(
        &mut self,
        method_names: Option<Vec<String>>,
        success_url: Option<String>,
        failure_url: Option<String>,
    ) {
        let contract_id = crate::config::CONTRACT_ID;
        self.wallet
            .request_sign_in(
                Some(contract_id.to_string()),
                method_names,
                success_url,
                failure_url,
            )
            .await;
    }

    pub fn disconnect(&mut self) {
        self.wallet.sign_out();
        self.checked = false;
    }

    pub fn sign_message(&self, message: &str) -> String {
        let wallet = self.wallet.clone();
        let signature = wallet.sign_message(message.as_bytes());

        signature.to_string()
    }
}
