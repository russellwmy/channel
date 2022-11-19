# User Smart Contract

Smart contract stores user info

## Commands

- `cargo make format` - Format the code
- `cargo make clean` - Remove the build directory
- `cargo make build` - Build the program
- `cargo make test` - Run the tests
- `cargo make deploy-contract` - Deploy the contract to blockchain
- `cargo make create-contract-account` - Create contract account
- `cargo make delete-contract-account` - Delete contract account, all data will be purged
- `cargo make init-contract` - Initialize smart contract, it only run once after create contract account

## Get Start
In common situation, we already create contract account and init the contract, unless we delete the contract, therefore developer can deploy the contract to `testnet` to update the contract for testing.

## Dev
- near login
- see test.sh