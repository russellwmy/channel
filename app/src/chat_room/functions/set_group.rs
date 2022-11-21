use std::str::FromStr;

use web3_anywhere::{
    crypto::Signature,
    near::{
        primitives::{
            actions::{Action, FunctionCallAction},
            transaction::{SignedTransaction, Transaction},
            types::{AccountId, BlockReference},
        },
        Wallet,
    },
};

use crate::{
    chat_room::types::SetGroupInput,
    config::{CONTRACT_ID, GAS_FEE},
};

pub async fn set_group(wallet: Wallet, input: SetGroupInput) {
    let contract_id = CONTRACT_ID.parse::<AccountId>().unwrap();
    let account_id = wallet.account_id().unwrap();
    let public_key = wallet.public_key().unwrap();
    let block = wallet
        .near_rpc_user()
        .get_block(BlockReference::latest())
        .await
        .unwrap();
    let access_key = wallet
        .near_rpc_user()
        .view_access_key(&account_id, &public_key)
        .await
        .unwrap();

    let block_hash = block.header.hash;
    let nonce = access_key.nonce + 1;
    let args = serde_json::json!(input);
    let bytes = serde_json::to_vec(&args).unwrap();

    let transaction = Transaction {
        signer_id: account_id,
        public_key,
        nonce: nonce,
        block_hash: block_hash,
        receiver_id: contract_id,
        actions: vec![Action::FunctionCall(FunctionCallAction {
            method_name: "set_group".to_string(),
            args: bytes.clone().into(),
            gas: GAS_FEE,
            deposit: 0,
        })],
    };
    let (hash, _) = transaction.clone().get_hash_and_size();
    let message = wallet.sign_message(hash.as_bytes());
    let signature = Signature::from_str(&message).unwrap();
    let tx = SignedTransaction::new(signature, transaction);
    wallet.near_rpc_user().send_transaction(tx).await;
}
