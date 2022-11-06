#!/bin/bash

cargo make build
near deploy -f 4a303e69-94d4-4319-a847-2ece362c016d.testnet target/wasm32-unknown-unknown/release/contract.wasm

near call "4a303e69-94d4-4319-a847-2ece362c016d.testnet" migrate "{}" --accountId "4a303e69-94d4-4319-a847-2ece362c016d.testnet"

near call "4a303e69-94d4-4319-a847-2ece362c016d.testnet" set_user "{ \"name\": \"fish\",\"data\": \"json encoded data??\" }" --accountId "4a303e69-94d4-4319-a847-2ece362c016d.testnet"

near call "4a303e69-94d4-4319-a847-2ece362c016d.testnet" set_group "{ \"uuid\": \"1\",\"name\": \"createdGroup\" }" --accountId "4a303e69-94d4-4319-a847-2ece362c016d.testnet"
near view "4a303e69-94d4-4319-a847-2ece362c016d.testnet" get_groups_debug
near call "4a303e69-94d4-4319-a847-2ece362c016d.testnet" set_group "{ \"uuid\": \"1\",\"name\": \"editedGroup\" }" --accountId "4a303e69-94d4-4319-a847-2ece362c016d.testnet"
near view "4a303e69-94d4-4319-a847-2ece362c016d.testnet" get_groups_debug
near view "4a303e69-94d4-4319-a847-2ece362c016d.testnet" get_joined_groups "{ \"accountId\": \"4a303e69-94d4-4319-a847-2ece362c016d.testnet\"}"

near view "4a303e69-94d4-4319-a847-2ece362c016d.testnet" get_group_users "{ \"groupUuid\": \"1\"}"

near call "4a303e69-94d4-4319-a847-2ece362c016d.testnet" set_user "{ \"name\": \"fish fish\",\"data\": \"json encoded data??\" }" --accountId "4a303e69-94d4-4319-a847-2ece362c016d.testnet"
near view "4a303e69-94d4-4319-a847-2ece362c016d.testnet" get_group_users "{ \"groupUuid\": \"1\"}"