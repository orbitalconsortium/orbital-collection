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
