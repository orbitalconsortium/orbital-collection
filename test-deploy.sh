#!/bin/bash
set -e  # Exit on error

# Text colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
ORBITAL_CONTRACT_PATH="./target/wasm32-unknown-unknown/release/orbitals_orbital_contract.wasm"

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
  echo "$1" | grep -oP 'txid: \K[a-f0-9]+'
}

# Check if the contract file exists
if [ ! -f "$ORBITAL_CONTRACT_PATH" ]; then
  echo -e "${RED}Error: Orbital contract file '$ORBITAL_CONTRACT_PATH' does not exist.${NC}"
  echo -e "${YELLOW}Make sure to build the contracts with 'cargo build --release --target wasm32-unknown-unknown'${NC}"
  exit 1
fi

# Initialize regtest environment
log_step "Initializing regtest environment"
oyl regtest init -p alkanes
log_success "Regtest environment initialized"

# Generate blocks
log_step "Generating blocks"
oyl regtest genBlocks -p alkanes
log_success "Blocks generated"

# Get addresses from mnemonic
log_step "Deriving addresses from mnemonic"

# Use a temporary file to store the output
TEMP_OUTPUT=$(mktemp)
# Note: The command doesn't accept -m option, using default mnemonic from provider
oyl account mnemonicToAccount -p alkanes > "$TEMP_OUTPUT" 2>&1

# Remove ANSI color codes and extract the JSON part
CLEAN_OUTPUT=$(cat "$TEMP_OUTPUT" | sed 's/\x1B\[[0-9;]*[mK]//g')
echo "$CLEAN_OUTPUT" > "$TEMP_OUTPUT"

# Extract addresses directly using grep and sed with more precise patterns
NATIVE_SEGWIT_ADDRESS=$(grep -A 3 "nativeSegwit" "$TEMP_OUTPUT" | grep "address:" | sed -E "s/.*address: '([^']+)'.*/\1/")

# Ensure we have clean addresses without any extra text
NATIVE_SEGWIT_ADDRESS=$(echo "$NATIVE_SEGWIT_ADDRESS" | tr -d ' \n\r\t')

# Clean up temporary file
rm "$TEMP_OUTPUT"

log_info "Native SegWit Address: $NATIVE_SEGWIT_ADDRESS"
log_success "Addresses derived"

# Fund the address with more sats
log_step "Funding address"
oyl regtest sendFromFaucet -t "$NATIVE_SEGWIT_ADDRESS" -p alkanes -s 100000000
log_success "Address funded with 100,000,000 sats"

# Generate blocks to confirm transactions (generate 6 blocks to ensure confirmation)
log_step "Generating blocks to confirm transactions"
for i in {1..6}; do
  oyl regtest genBlocks -p alkanes
  sleep 1
done
log_success "Blocks generated"

# Check account balance
log_step "Checking account balance"
oyl utxo accountUtxos -p alkanes
log_success "Account balance checked"

# Deploy orbital template
log_step "Deploying orbital template"
ORBITAL_OUTPUT=$(oyl alkane new-contract -c "$ORBITAL_CONTRACT_PATH" --calldata '3,57570,101' --feeRate 4 -p alkanes)
ORBITAL_TXID=$(extract_txid "$ORBITAL_OUTPUT")
log_info "Orbital template deployed with txid: $ORBITAL_TXID"

log_step "Test complete!"