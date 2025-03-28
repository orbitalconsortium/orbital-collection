use alkanes_runtime::{auth::AuthenticatedResponder, token::Token};
#[allow(unused_imports)]
use alkanes_runtime::{
    println,
    stdio::{stdout, Write},
};
use anyhow::{anyhow, Result};
use alkanes_runtime::{runtime::AlkaneResponder, storage::StoragePointer};
use alkanes_support::{id::AlkaneId, utils::{shift_or_err}};
use alkanes_support::{context::Context, parcel::AlkaneTransfer, response::CallResponse};
use metashrew_support::{utils::{consensus_decode}, compat::{to_arraybuffer_layout, to_passback_ptr}};
use metashrew_support::index_pointer::KeyValuePointer;
use protorune_support::{network::{to_address_str, NetworkParams, set_network},protostone::{Protostone}};
use ordinals::{Runestone, Artifact};
use bitcoin::{Script, OutPoint, Amount, TxOut, Transaction};
use bitcoin::hashes::{Hash};
use types_support::{Payment};
use std::sync::Arc;

/// SyntheticBitcoin (frBTC) is a synthetic representation of Bitcoin on the Subfrost protocol.
/// It allows users to wrap their BTC into frBTC and unwrap frBTC back to BTC.
/// The contract verifies Bitcoin transactions to ensure proper wrapping and unwrapping.
#[derive(Default)]
pub struct SyntheticBitcoin(());

/// Configure the network parameters for the Bitcoin network.
/// This function sets the appropriate network parameters based on the build features.
/// By default, it uses regtest parameters.
#[cfg(all(
    not(feature = "mainnet"),
    not(feature = "testnet"),
    not(feature = "luckycoin"),
    not(feature = "dogecoin"),
    not(feature = "bellscoin")
))]
pub fn configure_network() {
    println!("Configuring network for regtest");
    set_network(NetworkParams {
        bech32_prefix: String::from("bcrt"),
        p2pkh_prefix: 0x64,
        p2sh_prefix: 0xc4,
    });
}
#[cfg(feature = "mainnet")]
pub fn configure_network() {
    set_network(NetworkParams {
        bech32_prefix: String::from("bc"),
        p2sh_prefix: 0x05,
        p2pkh_prefix: 0x00,
    });
}
#[cfg(feature = "testnet")]
pub fn configure_network() {
    set_network(NetworkParams {
        bech32_prefix: String::from("tb"),
        p2pkh_prefix: 0x6f,
        p2sh_prefix: 0xc4,
    });
}
#[cfg(feature = "luckycoin")]
pub fn configure_network() {
    set_network(NetworkParams {
        bech32_prefix: String::from("lky"),
        p2pkh_hash: 0x2f,
        p2sh_hash: 0x05,
    });
}

#[cfg(feature = "dogecoin")]
pub fn configure_network() {
    set_network(NetworkParams {
        bech32_prefix: String::from("dc"),
        p2pkh_hash: 0x1e,
        p2sh_hash: 0x16,
    });
}
#[cfg(feature = "bellscoin")]
pub fn configure_network() {
    set_network(NetworkParams {
        bech32_prefix: String::from("bel"),
        p2pkh_hash: 0x19,
        p2sh_hash: 0x1e,
    });
}

/// MintableToken is a trait for tokens that can be minted.
/// It extends the Token trait and adds a mint function.
pub trait MintableToken: Token {
    /// Mint new tokens and return an AlkaneTransfer.
    ///
    /// # Arguments
    /// * `context` - The context of the call
    /// * `value` - The amount of tokens to mint
    ///
    /// # Returns
    /// An AlkaneTransfer representing the minted tokens
    fn mint(&self, context: &Context, value: u128) -> AlkaneTransfer {
        println!("Minting {} frBTC tokens", value);
        AlkaneTransfer {
            id: context.myself.clone(),
            value,
        }
    }
}

impl Token for SyntheticBitcoin {
    fn name(&self) -> String {
        String::from("SUBFROST BTC")
    }
    fn symbol(&self) -> String {
        String::from("frBTC")
    }
    fn decimals(&self) -> u8 {
        8 // Same as Bitcoin
    }
}
impl MintableToken for SyntheticBitcoin {}

impl AuthenticatedResponder for SyntheticBitcoin {}

impl SyntheticBitcoin {
  /// Get the storage pointer for the signer's script pubkey
  fn signer_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/signer")
  }
  
  /// Get the signer's script pubkey
  fn signer(&self) -> Vec<u8> {
    self.signer_pointer().get().as_ref().clone()
  }
  
  /// Set the signer's script pubkey from a transaction output
  ///
  /// # Arguments
  /// * `context` - The context of the call
  /// * `_vout` - The output index in the transaction
  ///
  /// # Returns
  /// Result indicating success or failure
  fn set_signer(&self, context: &Context, _vout: u32) -> Result<()> {
    println!("Setting signer from transaction output {}", _vout);
    let vout = _vout as usize;
    let tx = consensus_decode::<Transaction>(&mut std::io::Cursor::new(self.transaction()))?;
    
    if let Some(Artifact::Runestone(ref runestone)) = Runestone::decipher(&tx) {
      let protostones = Protostone::from_runestone(runestone)?;
      let message = &protostones[(context.vout as usize) - tx.output.len() - 1];
      
      if message.edicts.len() != 0 {
        return Err(anyhow!("message cannot contain edicts, only a pointer"));
      }
      
      let pointer = message
        .pointer
        .ok_or("")
        .map_err(|_| anyhow!("no pointer in message"))?;
        
      if pointer as usize >= tx.output.len() {
        return Err(anyhow!("pointer cannot be a protomessage"));
      }
      
      if pointer as usize == vout {
        return Err(anyhow!("pointer cannot be equal to output spendable by synthetic"));
      }
      
      self.signer_pointer().set(Arc::new(tx.output[vout as usize].script_pubkey.as_bytes().to_vec()));
      println!("Signer set successfully to script: {:?}", tx.output[vout as usize].script_pubkey);
      Ok(())
    } else {
      Err(anyhow!("unexpected condition: execution occurred with no Protostone present"))
    }
  }
  
  /// Check if a transaction has already been processed
  ///
  /// # Arguments
  /// * `tx` - The transaction to check
  ///
  /// # Returns
  /// Result indicating if the transaction is new (Ok) or already processed (Err)
  fn observe_transaction(&self, tx: &Transaction) -> Result<()> {
    let txid = tx.compute_txid();
    println!("Checking if transaction {} has been processed", txid);
    
    let mut ptr = StoragePointer::from_keyword("/seen/").select(&txid.as_byte_array().to_vec());
    if ptr.get().len() != 0 {
      Err(anyhow!("transaction already processed"))
    } else {
      ptr.set_value::<u8>(0x01);
      println!("Transaction {} marked as processed", txid);
      Ok(())
    }
  }
  
  /// Compute the total output value sent to the signer
  ///
  /// # Arguments
  /// * `tx` - The transaction to compute outputs for
  ///
  /// # Returns
  /// The total value sent to the signer
  fn compute_output(&self, tx: &Transaction) -> u128 {
    let signer = self.signer();
    let total = tx.output.iter().fold(0, |r: u128, v: &TxOut| -> u128 {
      if v.script_pubkey.as_bytes().to_vec() == signer {
        r + <u64 as Into<u128>>::into(v.value.to_sat())
      } else {
        r
      }
    });
    
    println!("Computed output value: {} satoshis", total);
    total
  }
  
  /// Get the amount of frBTC to burn from the incoming alkanes
  ///
  /// # Arguments
  /// * `context` - The context of the call
  ///
  /// # Returns
  /// The amount of frBTC to burn
  fn burn_input(&self, context: &Context) -> Result<u64> {
    let value = context.incoming_alkanes.0.iter()
      .find(|v| context.myself == v.id)
      .ok_or("").map_err(|_| anyhow!("must spend synthetics into message"))?
      .value.try_into()?;
      
    println!("Burn input amount: {} frBTC", value);
    Ok(value)
  }
  
  /// Burn frBTC and create a payment for unwrapping to BTC
  ///
  /// # Arguments
  /// * `context` - The context of the call
  /// * `vout` - The output index in the transaction
  ///
  /// # Returns
  /// The amount of frBTC burned
  fn burn(&self, context: &Context, vout: usize) -> Result<u64> {
    println!("Unwrapping frBTC to BTC, output index: {}", vout);
    let tx = consensus_decode::<Transaction>(&mut std::io::Cursor::new(self.transaction()))?;
    let txid = tx.compute_txid();
    println!("Transaction ID: {}", txid);

    if let Some(Artifact::Runestone(ref runestone)) = Runestone::decipher(&tx) {
      let protostones = Protostone::from_runestone(runestone)?;
      let message = &protostones[(context.vout as usize) - tx.output.len() - 1];
      
      if message.edicts.len() != 0 {
        return Err(anyhow!("message cannot contain edicts, only a pointer"));
      }
      
      let pointer = message
        .pointer
        .ok_or("")
        .map_err(|_| anyhow!("no pointer in message"))?;
        
      if pointer as usize >= tx.output.len() {
        return Err(anyhow!("pointer cannot be a protomessage"));
      }
      
      if pointer as usize == vout {
        return Err(anyhow!("pointer cannot be equal to output spendable by synthetic"));
      }
      
      let signer = self.signer();
      if signer != tx.output[vout].script_pubkey.as_bytes().to_vec() {
        return Err(anyhow!("signer pubkey must be targeted with supplementary output"));
      }
      
      let value = self.burn_input(context)?;
      
      // Create a payment record for the unwrap
      let payment = Payment {
        output: TxOut {
          script_pubkey: tx.output[pointer as usize].script_pubkey.clone(),
          value: Amount::from_sat(value)
        },
        spendable: OutPoint {
          txid,
          vout: vout.try_into()?
        }
      };
      
      // Store the payment record
      StoragePointer::from_keyword("/payments/byheight/")
        .select_value(self.height())
        .append(Arc::<Vec<u8>>::new(payment.serialize()?));
        
      println!("Successfully unwrapped {} frBTC to BTC", value);
      println!("Payment created for recipient: {:?}", tx.output[pointer as usize].script_pubkey);
      
      Ok(value)
    } else {
      Err(anyhow!("execution triggered unexpectedly -- no protostone"))
    }
  }
  
  /// Wrap BTC to frBTC by verifying a Bitcoin transaction
  ///
  /// # Arguments
  /// * `context` - The context of the call
  ///
  /// # Returns
  /// An AlkaneTransfer representing the minted frBTC
  fn exchange(&self, context: &Context) -> Result<AlkaneTransfer> {
    println!("Wrapping BTC to frBTC");
    let tx = consensus_decode::<Transaction>(&mut std::io::Cursor::new(self.transaction()))?;
    let txid = tx.compute_txid();
    println!("Transaction ID: {}", txid);
    
    // Check if the transaction has already been processed
    self.observe_transaction(&tx)?;
    
    // Compute the amount of BTC sent to the signer
    let payout = self.compute_output(&tx);
    println!("Wrapping {} satoshis to frBTC", payout);
    
    // Mint frBTC tokens
    let transfer = self.mint(&context, payout);
    println!("Successfully wrapped BTC to {} frBTC", transfer.value);
    
    Ok(transfer)
  }
  
  /// Get all pending payments at the current height
  ///
  /// # Returns
  /// A vector of serialized Payment objects
  fn get_pending_payments(&self) -> Vec<u8> {
    let payments = StoragePointer::from_keyword("/payments/byheight/")
      .select_value(self.height())
      .get_list()
      .into_iter()
      .fold(Vec::<u8>::new(), |r, v| {
        let mut result = Vec::<u8>::with_capacity(r.len() + v.len());
        result.extend(&r);
        result.extend(v.as_ref());
        result
      });
      
    println!("Retrieved {} bytes of pending payments", payments.len());
    payments
  }
}

impl AlkaneResponder for SyntheticBitcoin {
    fn execute(&self) -> Result<CallResponse> {
        configure_network();
        let context = self.context()?;
        let mut inputs = context.inputs.clone();
        let mut response: CallResponse = CallResponse::forward(&context.incoming_alkanes.clone());
        
        match shift_or_err(&mut inputs)? {
            /* initialize(u128) - Initialize the contract with auth tokens */
            0 => {
                println!("Initializing frBTC contract");
                let mut pointer = StoragePointer::from_keyword("/initialized");
                if pointer.get().len() == 0 {
                    let auth_token_units = shift_or_err(&mut inputs)?;
                    println!("Deploying auth token with {} units", auth_token_units);
                    
                    response
                        .alkanes
                        .0
                        .push(self.deploy_auth_token(auth_token_units)?);
                        
                    pointer.set(Arc::new(vec![0x01]));
                    println!("frBTC contract initialized successfully");
                    Ok(response)
                } else {
                    return Err(anyhow!("already initialized"));
                }
            },
            
            /* set_signer(u32) - Set the signer script pubkey */
            1 => {
                println!("Setting signer script pubkey");
                self.only_owner()?;
                let vout: u32 = shift_or_err(&mut inputs)?.try_into()?;
                self.set_signer(&context, vout)?;
                response.data = self.signer();
                println!("Signer set successfully");
                Ok(response)
            },
            
            /* wrap() - Wrap BTC to frBTC */
            2 => {
                println!("Wrapping BTC to frBTC");
                if self.signer().is_empty() {
                    return Err(anyhow!("signer not set"));
                }
                
                response.alkanes.0.push(self.exchange(&context)?);
                println!("Wrap successful");
                Ok(response)
            },
            
            /* unwrap(u32) - Unwrap frBTC to BTC */
            3 => {
                println!("Unwrapping frBTC to BTC");
                if context.caller.clone() != (AlkaneId { tx: 0, block: 0 }) {
                    return Err(anyhow!("must be called by EOA"));
                }
                
                if context.incoming_alkanes.0.len() != 1 {
                    return Err(anyhow!("must only send frBTC as input"));
                }
                
                let vout: usize = shift_or_err(&mut inputs)?.try_into()?;
                let burn_value = self.burn(&context, vout)?;
                
                let mut burn_response = CallResponse::default();
                burn_response.data = burn_value.to_le_bytes().to_vec();
                println!("Unwrap successful, burned {} frBTC", burn_value);
                Ok(burn_response)
            },
            
            /* get_signer() -> String - Get the signer address */
            100 => {
                println!("Getting signer address");
                if self.signer().is_empty() {
                    response.data = "Signer not set".as_bytes().to_vec();
                } else {
                    response.data = to_address_str(Script::from_bytes(self.signer_pointer().get().as_ref()))
                        .ok_or("").map_err(|_| anyhow!("invalid script"))?
                        .as_bytes().to_vec();
                }
                Ok(response)
            },
            
            /* get_pending_payments() -> Vec<u8> - Get pending payments */
            101 => {
                println!("Getting pending payments");
                let mut payments = CallResponse::forward(&context.incoming_alkanes);
                payments.data = self.get_pending_payments();
                Ok(payments)
            },
            
            /* name() -> String - Get token name */
            99 => {
                response.data = self.name().into_bytes().to_vec();
                Ok(response)
            },
            
            /* symbol() -> String - Get token symbol */
            100 => {
                response.data = self.symbol().into_bytes().to_vec();
                Ok(response)
            },
            
            /* decimals() -> u8 - Get token decimals */
            102 => {
                response.data = vec![self.decimals()];
                Ok(response)
            },
            
            // Legacy opcodes for backward compatibility
            77 => { // Legacy mint
                println!("Using legacy mint opcode (77), consider using opcode 2 instead");
                response.alkanes.0.push(self.exchange(&context)?);
                Ok(response)
            },
            
            78 => { // Legacy burn
                println!("Using legacy burn opcode (78), consider using opcode 3 instead");
                if context.caller.clone() != (AlkaneId { tx: 0, block: 0 }) {
                    return Err(anyhow!("must be called by EOA"));
                }
                
                if context.incoming_alkanes.0.len() != 1 {
                    return Err(anyhow!("must only send frBTC as input"));
                }
                
                let vout: usize = shift_or_err(&mut inputs)?.try_into()?;
                let burn_value = self.burn(&context, vout)?;
                
                let mut burn_response = CallResponse::default();
                burn_response.data = burn_value.to_le_bytes().to_vec();
                Ok(burn_response)
            },
            
            1001 => { // Legacy payments_at_height
                println!("Using legacy payments_at_height opcode (1001), consider using opcode 101 instead");
                let mut payments = CallResponse::forward(&context.incoming_alkanes);
                payments.data = self.get_pending_payments();
                Ok(payments)
            },
            
            _ => {
                Err(anyhow!("unrecognized opcode"))
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&SyntheticBitcoin::default().run());
    to_passback_ptr(&mut response)
}
