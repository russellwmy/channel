use std::io::Error;

use log::info;
use uuid::Uuid;
use web3_anywhere::near::{
    primitives::{
        actions::{Action, FunctionCallAction},
        crypto::PublicKey,
        query::FunctionArgs,
        transaction::Transaction,
        types::{AccountId, BlockReference},
    },
    Wallet,
};

use crate::{
    chatroom::types::SetGroupInput,
    config::{CHANNEL_CONTRACT_ID, GAS_FEE},
};

pub async fn set_group(wallet: Wallet, name: String) {
    let id = Uuid::new_v4();

    let input = SetGroupInput {
        uuid: "4".to_string(),
        name: name,
    };
    let contract_id = CHANNEL_CONTRACT_ID.parse::<AccountId>().unwrap();
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
    let nonce = access_key.nonce;
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
            args: bytes.into(),
            gas: GAS_FEE,
            deposit: GAS_FEE as u128,
        })],
    };

    wallet
        .request_sign_transactions(vec![transaction], None, None)
        .await;
}
