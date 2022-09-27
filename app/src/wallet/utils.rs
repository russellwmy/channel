use web3_anywhere::{
    key_man::{KeyStore, Signer},
    near::{NearConfig, Wallet},
};

pub fn connect_wallet() -> Wallet {
    let key_store = KeyStore::new_browser_local_storage_key_store();
    let signer = Signer::new_in_memory_signer(key_store.clone());
    let config = NearConfig {
        signer,
        key_store,
        network: "testnet".to_string(),
        wallet_url: "https://wallet.testnet.near.org".to_string(),
        helper_url: "https://helper.testnet.near.org".to_string(),
        explorer_url: "https://explorer.testnet.near.org".to_string(),
        node_url: "https://rpc.testnet.near.org".to_string(),
    };

    let wallet = Wallet::new(config);

    wallet
}
