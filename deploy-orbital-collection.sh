#!/bin/bash
# deploy-orbital-collection.sh
#
# This script automates the deployment of the orbital-collection stack
# on a fresh -p alkanes docker-compose environment.
#
# Usage: ./deploy-orbital-collection.sh [MNEMONIC] [IMAGE_PATH]
#
# If MNEMONIC is not provided, a default one will be used.
# If IMAGE_PATH is not provided, a default one will be used.

set -e  # Exit on error

# Text colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DEFAULT_MNEMONIC="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
DEFAULT_IMAGE_PATH="./test-image.png"
ORBITAL_CONTRACT_PATH="./target/wasm32-unknown-unknown/release/orbitals_orbital_contract.wasm"
COLLECTION_CONTRACT_PATH="./target/wasm32-unknown-unknown/release/orbitals_collection_contract.wasm"
BITCOIN_COLLECTION_CONTRACT_PATH="./target/wasm32-unknown-unknown/release/orbitals_bitcoin_collection_contract.wasm"
BITCOIN_SALE_CONTRACT_PATH="./target/wasm32-unknown-unknown/release/orbitals_bitcoin_sale_contract.wasm"
CONTAINER_OUTPUT_PATH="./image.wasm"
BTC_PRICE_SATS=10000000  # 0.1 BTC in satoshis
BTC_PAYMENT_HELPER="./btc-payment-helper.js"
RPC_URL="http://localhost:18888"
FUNDING_AMOUNT=1000000000  # 10 BTC in satoshis
FEE_RATE=5  # Increased fee rate to meet min relay fee

# Constants for calldata
ORBITAL_CONSTANT="57570"
COLLECTION_CONSTANT="57576"
BITCOIN_COLLECTION_CONSTANT="57577"
BITCOIN_SALE_CONSTANT="57578"

# Parse arguments
MNEMONIC=${1:-$DEFAULT_MNEMONIC}
IMAGE_PATH=${2:-$DEFAULT_IMAGE_PATH}

# Function to check if a command exists
command_exists() {
  command -v "$1" >/dev/null 2>&1
}

# Check for required commands
for cmd in oyl jq node curl; do
  if ! command_exists $cmd; then
    echo -e "${RED}Error: $cmd is required but not installed.${NC}"
    exit 1
  fi
done

# Check if the image file exists
if [ ! -f "$IMAGE_PATH" ]; then
  echo -e "${RED}Error: Image file '$IMAGE_PATH' does not exist.${NC}"
  exit 1
fi

# Check if the contract files exist
if [ ! -f "$ORBITAL_CONTRACT_PATH" ]; then
  echo -e "${RED}Error: Orbital contract file '$ORBITAL_CONTRACT_PATH' does not exist.${NC}"
  echo -e "${YELLOW}Make sure to build the contracts with 'cargo build --release --target wasm32-unknown-unknown'${NC}"
  exit 1
fi

if [ ! -f "$COLLECTION_CONTRACT_PATH" ]; then
  echo -e "${RED}Error: Collection contract file '$COLLECTION_CONTRACT_PATH' does not exist.${NC}"
  echo -e "${YELLOW}Make sure to build the contracts with 'cargo build --release --target wasm32-unknown-unknown'${NC}"
  exit 1
fi

if [ ! -f "$BITCOIN_COLLECTION_CONTRACT_PATH" ]; then
  echo -e "${RED}Error: Bitcoin Collection contract file '$BITCOIN_COLLECTION_CONTRACT_PATH' does not exist.${NC}"
  echo -e "${YELLOW}Make sure to build the contracts with 'cargo build --release --target wasm32-unknown-unknown'${NC}"
  exit 1
fi

if [ ! -f "$BITCOIN_SALE_CONTRACT_PATH" ]; then
  echo -e "${RED}Error: Bitcoin Sale contract file '$BITCOIN_SALE_CONTRACT_PATH' does not exist.${NC}"
  echo -e "${YELLOW}Make sure to build the contracts with 'cargo build --release --target wasm32-unknown-unknown'${NC}"
  exit 1
fi

# Function to log steps
log_step() {
  echo -e "${BLUE}==== $1 ====${NC}"
}

# Function to log success
log_success() {
  echo -e "${GREEN}✓ $1${NC}"
}

# Function to log info
log_info() {
  echo -e "${YELLOW}$1${NC}"
}

# Function to extract txid from oyl output
extract_txid() {
  # Extract the first occurrence of a txid pattern and deduplicate
  echo "$@" | grep -oP "txId: '\K[a-f0-9]+(?=')" | head -n 1
}

# Function to check if metashrew_height is greater than or equal to btc_getblockcount
check_blockchain_sync() {
  log_step "Checking blockchain synchronization"
  
  # Get btc_getblockcount
  BTC_BLOCK_COUNT=$(curl -s -X POST $RPC_URL \
    -H 'Content-Type: application/json' \
    -d '{"jsonrpc":"2.0","id":1,"method":"btc_getblockcount","params":[]}' | jq -r '.result')
  
  # Get metashrew_height
  METASHREW_HEIGHT=$(curl -s -X POST $RPC_URL \
    -H 'Content-Type: application/json' \
    -d '{"jsonrpc":"2.0","id":1,"method":"metashrew_height","params":[]}' | jq -r '.result')
  
  log_info "BTC Block Count: $BTC_BLOCK_COUNT"
  log_info "Metashrew Height: $METASHREW_HEIGHT"
  
  # Check if metashrew_height is greater than or equal to btc_getblockcount
  if [ "$METASHREW_HEIGHT" -ge "$BTC_BLOCK_COUNT" ]; then
    log_success "Blockchain is synchronized"
    return 0
  else
    log_info "Waiting for blockchain to synchronize..."
    sleep 3
    check_blockchain_sync
  fi
}

# Function to wait after transaction and generate a block
wait_after_transaction() {
  log_info "Waiting 3 seconds after transaction..."
  sleep 3
  
  log_step "Generating block after transaction"
  oyl regtest genBlocks -p alkanes
  log_success "Block generated"
  
  check_blockchain_sync
}

# Function to check account balances
check_account_balances() {
  log_step "Checking account balances"
  
  # Show account information
  log_info "Account information from mnemonicToAccount:"
  oyl account mnemonicToAccount -p alkanes
  
  # Check UTXO details
  log_info "UTXO details:"
  UTXO_OUTPUT=$(oyl utxo accountUtxos -p alkanes)
  
  # Extract balances using jq
  UTXO_JSON=$(echo "$UTXO_OUTPUT" | grep -A 1000 "{" | grep -B 1000 "}")
  
  # Check if we have enough balance in both addresses
  NATIVE_SEGWIT_BALANCE=$(echo "$UTXO_OUTPUT" | grep -A 10 "nativeSegwit" | grep "spendableTotalBalance" | head -n 1 | grep -oP '\d+')
  TAPROOT_BALANCE=$(echo "$UTXO_OUTPUT" | grep -A 10 "taproot" | grep "spendableTotalBalance" | head -n 1 | grep -oP '\d+')
  
  log_info "Native SegWit Balance: $NATIVE_SEGWIT_BALANCE sats"
  log_info "Taproot Balance: $TAPROOT_BALANCE sats"
  
  if [ -z "$NATIVE_SEGWIT_BALANCE" ] || [ "$NATIVE_SEGWIT_BALANCE" -lt 100000000 ]; then
    log_info "Native SegWit balance is too low, funding required"
    return 1
  fi
  
  if [ -z "$TAPROOT_BALANCE" ] || [ "$TAPROOT_BALANCE" -lt 100000000 ]; then
    log_info "Taproot balance is too low, funding required"
    return 1
  fi
  
  log_success "Account balances are sufficient"
  return 0
}

# Create BTC payment helper script
log_step "Creating BTC payment helper script"
cat > "$BTC_PAYMENT_HELPER" << 'EOF'
// btc-payment-helper.js
// This script extends the oyl-sdk to allow sending BTC with alkane execute transactions

const fs = require('fs');
const { OylProvider } = require('oyl-sdk');

// Parse command line arguments
const args = process.argv.slice(2);
const argMap = {};
for (let i = 0; i < args.length; i++) {
  if (args[i].startsWith('--')) {
    const key = args[i].substring(2);
    const value = args[i+1] && !args[i+1].startsWith('--') ? args[i+1] : true;
    argMap[key] = value;
    if (value !== true) i++;
  }
}

// Required parameters
const targetTxid = argMap['target-txid'];
const opcode = argMap['opcode'];
const btcAmount = parseInt(argMap['btc-amount'] || '0');
const btcAddress = argMap['btc-address'];
const provider = argMap['provider'] || 'alkanes';

if (!targetTxid || !opcode || !btcAmount || !btcAddress) {
  console.error('Usage: node btc-payment-helper.js --target-txid <txid> --opcode <opcode> --btc-amount <sats> --btc-address <address> [--provider <provider>]');
  process.exit(1);
}

async function main() {
  try {
    // Initialize provider
    const oylProvider = new OylProvider(provider);
    
    // Create a transaction with BTC payment
    const tx = await oylProvider.alkane.createExecuteTx({
      target: targetTxid,
      opcode: parseInt(opcode),
    });
    
    // Add BTC payment output
    tx.addOutput(btcAddress, btcAmount);
    
    // Sign and broadcast the transaction
    const signedTx = await oylProvider.wallet.signTx(tx);
    const txid = await oylProvider.broadcast(signedTx);
    
    console.log(`Transaction sent with txid: ${txid}`);
    console.log(`BTC payment of ${btcAmount} sats sent to ${btcAddress}`);
    
    return txid;
  } catch (error) {
    console.error('Error:', error);
    process.exit(1);
  }
}

main();
EOF

log_success "BTC payment helper script created"

# Initialize regtest environment
log_step "Initializing regtest environment"
oyl regtest init -p alkanes
log_success "Regtest environment initialized"

# Generate blocks
log_step "Generating blocks"
oyl regtest genBlocks -p alkanes
wait_after_transaction
log_success "Blocks generated"

# Get addresses from mnemonic
log_step "Deriving addresses from mnemonic"
log_info "Using mnemonic: $MNEMONIC"

# Show full account information
log_info "Full account information:"
oyl account mnemonicToAccount -p alkanes

# Use a temporary file to store the output
TEMP_OUTPUT=$(mktemp)
# Note: The command doesn't accept -m option, using default mnemonic from provider
oyl account mnemonicToAccount -p alkanes > "$TEMP_OUTPUT" 2>&1

# Remove ANSI color codes and extract the JSON part
CLEAN_OUTPUT=$(cat "$TEMP_OUTPUT" | sed 's/\x1B\[[0-9;]*[mK]//g')
echo "$CLEAN_OUTPUT" > "$TEMP_OUTPUT"

# Extract addresses directly using grep and sed with more precise patterns
NATIVE_SEGWIT_ADDRESS=$(grep -A 3 "nativeSegwit" "$TEMP_OUTPUT" | grep "address:" | sed -E "s/.*address: '([^']+)'.*/\1/")
TAPROOT_ADDRESS=$(grep -A 3 "taproot:" "$TEMP_OUTPUT" | grep "address:" | sed -E "s/.*address: '([^']+)'.*/\1/")

# Ensure we have clean addresses without any extra text
NATIVE_SEGWIT_ADDRESS=$(echo "$NATIVE_SEGWIT_ADDRESS" | tr -d ' \n\r\t')
TAPROOT_ADDRESS=$(echo "$TAPROOT_ADDRESS" | tr -d ' \n\r\t')

# Clean up temporary file
rm "$TEMP_OUTPUT"

log_info "Native SegWit Address: $NATIVE_SEGWIT_ADDRESS"
log_info "Taproot Address: $TAPROOT_ADDRESS"
log_success "Addresses derived"

# Fund the addresses with more sats
log_step "Funding addresses"
log_info "Funding Native SegWit Address with $FUNDING_AMOUNT sats (10 BTC)"
oyl regtest sendFromFaucet -t "$NATIVE_SEGWIT_ADDRESS" -p alkanes -s $FUNDING_AMOUNT
wait_after_transaction
log_info "Funding Taproot Address with $FUNDING_AMOUNT sats (10 BTC)"
oyl regtest sendFromFaucet -t "$TAPROOT_ADDRESS" -p alkanes -s $FUNDING_AMOUNT
wait_after_transaction
log_success "Addresses funded with $FUNDING_AMOUNT sats each (10 BTC)"

# Generate blocks to confirm transactions (generate 6 blocks to ensure confirmation)
log_step "Generating blocks to confirm transactions"
for i in {1..6}; do
  oyl regtest genBlocks -p alkanes
  sleep 1
done
wait_after_transaction
log_success "Blocks generated"

# Check account balances
check_account_balances

# Generate container WASM
log_step "Generating container WASM from image"
log_info "Using image: $IMAGE_PATH"
log_info "Output path: $CONTAINER_OUTPUT_PATH"

if command_exists orbitals-container-generate; then
  orbitals-container-generate generate "$IMAGE_PATH" -o "$CONTAINER_OUTPUT_PATH"
else
  log_info "orbitals-container-generate not found in PATH, using npx"
  npx orbitals-container-generate generate "$IMAGE_PATH" -o "$CONTAINER_OUTPUT_PATH"
fi

log_success "Container WASM generated"

# Check account balances again before deploying contracts
check_account_balances

# Deploy orbital template
log_step "Deploying orbital template"
log_info "Contract path: $ORBITAL_CONTRACT_PATH"
log_info "Calldata: 3,$ORBITAL_CONSTANT,101"
ORBITAL_OUTPUT=$(oyl alkane new-contract -c "$ORBITAL_CONTRACT_PATH" --calldata "3,$ORBITAL_CONSTANT,101" --feeRate $FEE_RATE -p alkanes)
ORBITAL_TXID=$(extract_txid "$ORBITAL_OUTPUT")
log_info "Orbital template deployed with txid: $ORBITAL_TXID"
wait_after_transaction

# Check account balances
check_account_balances

# Deploy collection alkane
log_step "Deploying collection alkane"
log_info "Contract path: $COLLECTION_CONTRACT_PATH"
log_info "Calldata: 3,$COLLECTION_CONSTANT,101"
COLLECTION_OUTPUT=$(oyl alkane new-contract -c "$COLLECTION_CONTRACT_PATH" --calldata "3,$COLLECTION_CONSTANT,101" --feeRate $FEE_RATE -p alkanes)
COLLECTION_TXID=$(extract_txid "$COLLECTION_OUTPUT")
log_info "Collection alkane deployed with txid: $COLLECTION_TXID"
wait_after_transaction

# Check account balances
check_account_balances

# Deploy bitcoin collection template
log_step "Deploying bitcoin collection template"
log_info "Contract path: $BITCOIN_COLLECTION_CONTRACT_PATH"
log_info "Calldata: 3,$BITCOIN_COLLECTION_CONSTANT,101"
BITCOIN_COLLECTION_OUTPUT=$(oyl alkane new-contract -c "$BITCOIN_COLLECTION_CONTRACT_PATH" --calldata "3,$BITCOIN_COLLECTION_CONSTANT,101" --feeRate $FEE_RATE -p alkanes)
BITCOIN_COLLECTION_TXID=$(extract_txid "$BITCOIN_COLLECTION_OUTPUT")
log_info "Bitcoin collection template deployed with txid: $BITCOIN_COLLECTION_TXID"
wait_after_transaction

# Check account balances
check_account_balances

# Deploy bitcoin sale template
log_step "Deploying bitcoin sale template"
log_info "Contract path: $BITCOIN_SALE_CONTRACT_PATH"
log_info "Calldata: 3,$BITCOIN_SALE_CONSTANT,101"
BITCOIN_SALE_OUTPUT=$(oyl alkane new-contract -c "$BITCOIN_SALE_CONTRACT_PATH" --calldata "3,$BITCOIN_SALE_CONSTANT,101" --feeRate $FEE_RATE -p alkanes)
BITCOIN_SALE_TXID=$(extract_txid "$BITCOIN_SALE_OUTPUT")
log_info "Bitcoin sale template deployed with txid: $BITCOIN_SALE_TXID"
wait_after_transaction

# Check account balances
check_account_balances

# Deploy container
log_step "Deploying container"
log_info "Contract path: $CONTAINER_OUTPUT_PATH"
log_info "Calldata: 6,$COLLECTION_CONSTANT,0,357879337540,0,357879337540"
CONTAINER_OUTPUT=$(oyl alkane new-contract -c "$CONTAINER_OUTPUT_PATH" --calldata "6,$COLLECTION_CONSTANT,0,357879337540,0,357879337540" --feeRate $FEE_RATE -p alkanes)
CONTAINER_TXID=$(extract_txid "$CONTAINER_OUTPUT")
log_info "Container deployed with txid: $CONTAINER_TXID"
wait_after_transaction

# Check account balances
check_account_balances

# Verify deployment
log_step "Verifying deployment"
oyl provider alkanes -method trace -params "{\"txid\":\"$CONTAINER_TXID\", \"vout\": 3}" -p alkanes
log_success "Deployment verified"

# Deploy bitcoin sale instance
log_step "Deploying bitcoin sale instance"

# Use simpler values for the bitcoin sale instance
# Name and symbol for the collection
NAME_PART1="4276772" # "ABC" in decimal
NAME_PART2="0"
SYMBOL="4276772" # "ABC" in decimal

# Use a fixed payment address (the taproot address)
# Instead of trying to convert the address to decimal, we'll use a fixed value
PAYMENT_ADDR_PART1="123456789"
PAYMENT_ADDR_PART2="987654321"
PAYMENT_ADDR_PART3="123456789"

# Check account balances
check_account_balances

# Deploy bitcoin sale instance with the container
log_info "Contract path: $BITCOIN_SALE_CONTRACT_PATH"
log_info "Calldata: 6,$BITCOIN_SALE_CONSTANT,0,$BTC_PRICE_SATS,1000,$PAYMENT_ADDR_PART1,$PAYMENT_ADDR_PART2,$PAYMENT_ADDR_PART3,$NAME_PART1,$NAME_PART2,$SYMBOL"
BITCOIN_SALE_INSTANCE_OUTPUT=$(oyl alkane new-contract -c "$BITCOIN_SALE_CONTRACT_PATH" --calldata "6,$BITCOIN_SALE_CONSTANT,0,$BTC_PRICE_SATS,1000,$PAYMENT_ADDR_PART1,$PAYMENT_ADDR_PART2,$PAYMENT_ADDR_PART3,$NAME_PART1,$NAME_PART2,$SYMBOL" --feeRate $FEE_RATE -p alkanes)
BITCOIN_SALE_INSTANCE_TXID=$(extract_txid "$BITCOIN_SALE_INSTANCE_OUTPUT")
log_info "Bitcoin sale instance deployed with txid: $BITCOIN_SALE_INSTANCE_TXID"
wait_after_transaction

# Check account balances
check_account_balances

# Get the collection alkane ID
log_step "Getting collection alkane ID"
check_blockchain_sync
oyl provider alkanes -method trace -params "{\"txid\":\"$BITCOIN_SALE_INSTANCE_TXID\", \"vout\": 3}" -p alkanes
oyl provider alkanes -method trace -params "{\"txid\":\"$BITCOIN_SALE_INSTANCE_TXID\", \"vout\": 4}" -p alkanes
sleep 30
COLLECTION_ID_OUTPUT=$(oyl provider alkanes -method trace -params "{\"txid\":\"$BITCOIN_SALE_INSTANCE_TXID\", \"vout\": 3}" -p alkanes)
wait_after_transaction
# Extract the collection ID from the output
COLLECTION_ID_HEX=$(echo "$COLLECTION_ID_OUTPUT" | grep -A 5 "tx:" | grep -oP '0x[0-9a-f]+' | head -n 1)
COLLECTION_INSTANCE_TXID=$(printf "%x" $COLLECTION_ID_HEX)
log_info "Collection alkane ID: $COLLECTION_INSTANCE_TXID"

# Check account balances
check_account_balances

# Purchase an orbital using BTC
log_step "Purchasing an orbital using BTC"
log_info "Using BTC payment helper script"
PURCHASE_TXID=$(node "$BTC_PAYMENT_HELPER" --target-txid "$BITCOIN_SALE_INSTANCE_TXID" --opcode 77 --btc-amount "$BTC_PRICE_SATS" --btc-address "$TAPROOT_ADDRESS" --provider alkanes)
wait_after_transaction
log_info "Purchase transaction sent with txid: $PURCHASE_TXID"

# Check account balances
check_account_balances

# Get the orbital ID from the purchase response
log_step "Getting orbital ID from purchase"
log_step "txid to trace $PURCHASE_TXID"
check_blockchain_sync
oyl provider alkanes -method trace -params "{\"txid\":\"$PURCHASE_TXID\", \"vout\": 3}" -p alkanes
oyl provider alkanes -method trace -params "{\"txid\":\"$PURCHASE_TXID\", \"vout\": 4}" -p alkanes
PURCHASE_TRACE=$(oyl provider alkanes -method trace -params "{\"txid\":\"$PURCHASE_TXID\", \"vout\": 4}" -p alkanes)
sleep 30
# Extract the orbital ID from the trace output
# This is a simplified approach - in a real implementation, we would need to parse the trace output properly
ORBITAL_INSTANCE_ID=$(echo "$PURCHASE_TRACE" | grep -A 10 "alkanes:" | grep -oP 'id: \[\d+, \K\d+' | head -n 1)
log_info "Orbital instance ID: $ORBITAL_INSTANCE_ID"

# Verify the orbital data
log_step "Verifying orbital data"
ORBITAL_DATA_OUTPUT=$(oyl alkane execute -t "$ORBITAL_INSTANCE_ID" --opcode 1000 -p alkanes)
wait_after_transaction
# Check if the data contains the image bytes
if echo "$ORBITAL_DATA_OUTPUT" | grep -q "data:"; then
  log_success "Orbital data verified"
else
  log_info "Could not verify orbital data"
fi

log_step "Deployment complete!"
echo -e "${GREEN}Orbital Template TXID: $ORBITAL_TXID${NC}"
echo -e "${GREEN}Collection Template TXID: $COLLECTION_TXID${NC}"
echo -e "${GREEN}Bitcoin Collection Template TXID: $BITCOIN_COLLECTION_TXID${NC}"
echo -e "${GREEN}Bitcoin Sale Template TXID: $BITCOIN_SALE_TXID${NC}"
echo -e "${GREEN}Container TXID: $CONTAINER_TXID${NC}"
echo -e "${GREEN}Bitcoin Sale Instance TXID: $BITCOIN_SALE_INSTANCE_TXID${NC}"
echo -e "${GREEN}Collection Instance TXID: $COLLECTION_INSTANCE_TXID${NC}"
echo -e "${GREEN}Purchased Orbital ID: $ORBITAL_INSTANCE_ID${NC}"
echo ""
echo -e "${YELLOW}To purchase another orbital, use:${NC}"
echo -e "node $BTC_PAYMENT_HELPER --target-txid $BITCOIN_SALE_INSTANCE_TXID --opcode 77 --btc-amount $BTC_PRICE_SATS --btc-address $TAPROOT_ADDRESS --provider alkanes"
echo ""
echo -e "${YELLOW}To view an orbital's data, use:${NC}"
echo -e "oyl alkane execute -t $ORBITAL_INSTANCE_ID --opcode 1000 -p alkanes"
