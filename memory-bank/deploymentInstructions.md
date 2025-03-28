# Orbital Collection Deployment Instructions

This document provides instructions for deploying the Orbital Collection system, including the Bitcoin-based collection and sale alkanes.

## Prerequisites

Before deploying, ensure you have the following installed:

- Rust and Cargo
- wasm32-unknown-unknown target (`rustup target add wasm32-unknown-unknown`)
- Node.js and npm
- oyl-sdk CLI
- jq (for JSON processing)
- curl (for JSON-RPC requests)

## Building the Contracts

Build all contracts with the following command:

```bash
cargo build --release --target wasm32-unknown-unknown
```

This will generate the following WASM files:

- `./target/wasm32-unknown-unknown/release/orbitals_orbital_contract.wasm`
- `./target/wasm32-unknown-unknown/release/orbitals_collection_contract.wasm`
- `./target/wasm32-unknown-unknown/release/orbitals_bitcoin_collection_contract.wasm`
- `./target/wasm32-unknown-unknown/release/orbitals_bitcoin_sale_contract.wasm`

## Generating the Container WASM

To generate the container WASM from an image:

```bash
npx orbitals-container-generate generate ./path/to/image.png -o ./image.wasm
```

## Automated Deployment

The `deploy-orbital-collection.sh` script automates the entire deployment process. It performs the following steps:

1. Creates a BTC payment helper script
2. Initializes the regtest environment
3. Generates blocks
4. Derives addresses from the mnemonic
5. Funds both the Native SegWit and Taproot addresses
6. Generates blocks to confirm transactions
7. Checks account balance
8. Generates container WASM from the provided image
9. Deploys the orbital template
10. Deploys the collection alkane
11. Deploys the bitcoin collection template
12. Deploys the bitcoin sale template
13. Deploys the container
14. Verifies the deployment
15. Deploys the bitcoin sale instance
16. Gets the collection alkane ID
17. Purchases an orbital using BTC
18. Checks balances
19. Gets the orbital ID from the purchase response
20. Verifies the orbital data

### Usage

```bash
./deploy-orbital-collection.sh [MNEMONIC] [IMAGE_PATH]
```

If MNEMONIC is not provided, a default one will be used.
If IMAGE_PATH is not provided, `./test-image.png` will be used.

### Blockchain Synchronization

The script includes a function to check if the blockchain is properly synchronized. This function:

1. Makes a JSON-RPC request to get the current block count (`btc_getblockcount`)
2. Makes a JSON-RPC request to get the current metashrew height (`metashrew_height`)
3. Ensures that the metashrew height is greater than or equal to the block count

This check is performed after each transaction to ensure that the blockchain is properly synchronized before proceeding to the next step. If the blockchain is not synchronized, the script will wait and check again.

### Transaction Timing

The script includes a 3-second wait between each command that issues a transaction. This helps ensure that transactions are properly processed and confirmed before proceeding to the next step.

### Known Issues

The script may encounter an "Insufficient Balance" error when deploying contracts. This is a known issue with the oyl-sdk. To resolve this:

1. Ensure both the Native SegWit and Taproot addresses are funded
2. Generate enough blocks to confirm the transactions
3. Check the account balance to ensure the funds are available

## Manual Deployment

If you prefer to deploy manually or if the automated script encounters issues, you can use the following commands:

### 1. Deploy Orbital Template

```bash
oyl alkane new-contract -c "./target/wasm32-unknown-unknown/release/orbitals_orbital_contract.wasm" --calldata '3,57570,101' --feeRate 4 -p alkanes
```

### 2. Deploy Collection Template

```bash
oyl alkane new-contract -c "./target/wasm32-unknown-unknown/release/orbitals_collection_contract.wasm" --calldata '3,57576,101' --feeRate 4 -p alkanes
```

### 3. Deploy Bitcoin Collection Template

```bash
oyl alkane new-contract -c "./target/wasm32-unknown-unknown/release/orbitals_bitcoin_collection_contract.wasm" --calldata '3,57577,101' --feeRate 4 -p alkanes
```

### 4. Deploy Bitcoin Sale Template

```bash
oyl alkane new-contract -c "./target/wasm32-unknown-unknown/release/orbitals_bitcoin_sale_contract.wasm" --calldata '3,57578,101' --feeRate 4 -p alkanes
```

### 5. Deploy Container

```bash
oyl alkane new-contract -c "./image.wasm" --calldata "6,<COLLECTION_TXID>,0,357879337540,0,357879337540" --feeRate 4 -p alkanes
```

### 6. Deploy Bitcoin Sale Instance

```bash
oyl alkane new-contract -c "./target/wasm32-unknown-unknown/release/orbitals_bitcoin_sale_contract.wasm" --calldata "6,<BITCOIN_SALE_TXID>,0,<BTC_PRICE_SATS>,1000,<TAPROOT_PART1>,<TAPROOT_PART2>,<TAPROOT_PART3>,<NAME_PART1>,<NAME_PART2>,<SYMBOL>" --feeRate 4 -p alkanes
```

### 7. Purchase an Orbital Using BTC

```bash
node ./btc-payment-helper.js --target-txid <BITCOIN_SALE_INSTANCE_TXID> --opcode 77 --btc-amount <BTC_PRICE_SATS> --btc-address <TAPROOT_ADDRESS> --provider alkanes
```

### 8. View Orbital Data

```bash
oyl alkane execute -t <ORBITAL_INSTANCE_ID> --opcode 1000 -p alkanes
```

## BTC Payment Helper Script

The deployment script creates a `btc-payment-helper.js` file that extends the oyl-sdk to allow sending BTC with alkane execute transactions. This script is used to purchase orbitals from the bitcoin sale alkane.

### Usage

```bash
node btc-payment-helper.js --target-txid <txid> --opcode <opcode> --btc-amount <sats> --btc-address <address> [--provider <provider>]
```

## Checking Blockchain Synchronization Manually

You can check if the blockchain is properly synchronized by making the following JSON-RPC requests:

```bash
# Get the current block count
curl -s -X POST http://localhost:18888 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"btc_getblockcount","params":[]}'

# Get the current metashrew height
curl -s -X POST http://localhost:18888 \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"metashrew_height","params":[]}'
```

The metashrew height should be greater than or equal to the block count.

## Troubleshooting

### Insufficient Balance Error

If you encounter an "Insufficient Balance" error when deploying contracts:

1. Check your account balance with `oyl utxo accountUtxos -p alkanes`
2. Ensure both the Native SegWit and Taproot addresses are funded
3. Generate more blocks to confirm transactions with `oyl regtest genBlocks -p alkanes`
4. Try deploying again

### Contract Deployment Failures

If contract deployment fails:

1. Check that the contract WASM files exist and are correctly built
2. Ensure you have enough funds in your account
3. Try deploying with a lower fee rate
4. Check the oyl-sdk logs for more information

### Blockchain Synchronization Issues

If you encounter blockchain synchronization issues:

1. Check the blockchain synchronization status using the JSON-RPC requests described above
2. If the metashrew height is less than the block count, wait for it to catch up
3. If the blockchain is not synchronizing, try restarting the regtest environment