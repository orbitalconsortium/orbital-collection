use alkanes_runtime::declare_alkane;
use alkanes_runtime::message::MessageDispatch;
use alkanes_runtime::{runtime::AlkaneResponder, storage::StoragePointer};
use alkanes_support::{parcel::AlkaneTransfer, response::CallResponse};
use anyhow::{anyhow, Result};
use metashrew_support::compat::to_arraybuffer_layout;
use metashrew_support::index_pointer::KeyValuePointer;
use alkanes_support::utils::overflow_error;
use alkanes_support::id::AlkaneId;
use alkanes_support::parcel::AlkaneTransferParcel;
use alkanes_support::cellpack::Cellpack;
use std::sync::Arc;
use bitcoin::{Script, TxOut, Transaction};
use bitcoin::hashes::Hash;
use metashrew_support::utils::consensus_decode;
use protorune_support::network::{to_address_str, NetworkParams, set_network};

/// BitcoinSale alkane for selling orbital instances using BTC payments
#[derive(Default)]
pub struct BitcoinSale(());

/// Bitcoin collection template ID - this is the template used for creating the collection
pub const BITCOIN_COLLECTION_TEMPLATE_ID: u128 = 0xe0e4;

/// Message enum for opcode-based dispatch
#[derive(MessageDispatch)]
enum BitcoinSaleMessage {
    /// Initialize the sale
    #[opcode(0)]
    Initialize {
        /// Price per orbital in satoshis
        price: u128,
        /// Maximum number of orbitals that can be sold
        limit: u128,
        /// Taproot address part 1 (first 10 bytes)
        taproot_part1: u128,
        /// Taproot address part 2 (next 10 bytes)
        taproot_part2: u128,
        /// Taproot address part 3 (last 12 bytes)
        taproot_part3: u128,
        /// Collection name part 1
        name_part1: u128,
        /// Collection name part 2
        name_part2: u128,
        /// Collection symbol
        symbol: u128,
    },

    /// Purchase an orbital
    #[opcode(77)]
    Purchase,

    /// Get the collection alkane ID
    #[opcode(99)]
    #[returns(Vec<u8>)]
    GetCollectionAlkaneId,

    /// Get the price per orbital
    #[opcode(101)]
    #[returns(u128)]
    GetPrice,

    /// Get the maximum number of orbitals that can be sold
    #[opcode(102)]
    #[returns(u128)]
    GetLimit,

    /// Get the number of orbitals sold
    #[opcode(103)]
    #[returns(u128)]
    GetSold,

    /// Get the terms of service
    #[opcode(104)]
    #[returns(String)]
    GetTermsOfService,

    /// Get the taproot address
    #[opcode(105)]
    #[returns(String)]
    GetTaprootAddress,
    
    /// Get the beneficiary address (view function)
    #[opcode(10010)]
    #[returns(String)]
    GetBeneficiary,
}

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
        p2pkh_prefix: 0x2f,
        p2sh_prefix: 0x05,
    });
}

#[cfg(feature = "dogecoin")]
pub fn configure_network() {
    set_network(NetworkParams {
        bech32_prefix: String::from("dc"),
        p2pkh_prefix: 0x1e,
        p2sh_prefix: 0x16,
    });
}

#[cfg(feature = "bellscoin")]
pub fn configure_network() {
    set_network(NetworkParams {
        bech32_prefix: String::from("bel"),
        p2pkh_prefix: 0x19,
        p2sh_prefix: 0x1e,
    });
}

impl BitcoinSale {
    /// Get the pointer to the collection alkane ID
    pub fn collection_alkane_id_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/collection-alkane-id")
    }

    /// Get the collection alkane ID
    pub fn collection_alkane_id(&self) -> Result<AlkaneId> {
        let data = self.collection_alkane_id_pointer().get();
        if data.len() == 0 {
            return Err(anyhow!("Collection alkane ID not found"));
        }
        
        // Deserialize the AlkaneId from storage
        let bytes = data.as_ref();
        if bytes.len() < 32 {
            return Err(anyhow!("Invalid collection alkane ID data"));
        }
        
        Ok(AlkaneId {
            block: u128::from_le_bytes(bytes[0..16].try_into().map_err(|_| anyhow!("Failed to parse block ID"))?),
            tx: u128::from_le_bytes(bytes[16..32].try_into().map_err(|_| anyhow!("Failed to parse tx ID"))?),
        })
    }

    /// Set the collection alkane ID
    pub fn set_collection_alkane_id(&self, id: &AlkaneId) {
        // Serialize the AlkaneId to bytes
        let mut bytes = Vec::with_capacity(32);
        bytes.extend_from_slice(&id.block.to_le_bytes());
        bytes.extend_from_slice(&id.tx.to_le_bytes());
        
        self.collection_alkane_id_pointer().set(Arc::new(bytes));
    }

    /// Get the pointer to the taproot address
    pub fn taproot_address_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/taproot-address")
    }

    /// Get the taproot address script
    pub fn taproot_address_script(&self) -> Vec<u8> {
        self.taproot_address_pointer().get().as_ref().clone()
    }

    /// Set the taproot address from three u128 parts
    pub fn set_taproot_address(&self, part1: u128, part2: u128, part3: u128) {
        // Combine the three parts to form a 32-byte address
        let mut address_bytes = Vec::with_capacity(32);
        
        // Extract the first 10 bytes from part1
        let part1_bytes = part1.to_le_bytes();
        address_bytes.extend_from_slice(&part1_bytes[0..10]);
        
        // Extract the next 10 bytes from part2
        let part2_bytes = part2.to_le_bytes();
        address_bytes.extend_from_slice(&part2_bytes[0..10]);
        
        // Extract the last 12 bytes from part3
        let part3_bytes = part3.to_le_bytes();
        address_bytes.extend_from_slice(&part3_bytes[0..12]);
        
        // Create a simple script that just pushes the address bytes
        // This is a simplified approach - in a real implementation, 
        // we would use proper taproot script creation
        let mut script_bytes = Vec::new();
        script_bytes.push(0x51); // OP_PUSHBYTES_32
        script_bytes.extend_from_slice(&address_bytes);
        
        // Store the script
        self.taproot_address_pointer().set(Arc::new(script_bytes));
    }

    /// Get the taproot address as a string
    pub fn taproot_address(&self) -> String {
        let script_bytes = self.taproot_address_script();
        if script_bytes.is_empty() {
            return String::from("Taproot address not set");
        }
        
        let script = Script::from_bytes(&script_bytes);
        to_address_str(script).unwrap_or_else(|| String::from("Invalid taproot address"))
    }

    /// Get the pointer to the price
    pub fn price_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/price")
    }

    /// Get the price per orbital in satoshis
    pub fn price(&self) -> u128 {
        self.price_pointer().get_value::<u128>()
    }

    /// Set the price per orbital in satoshis
    pub fn set_price(&self, price: u128) {
        self.price_pointer().set_value::<u128>(price);
    }

    /// Get the pointer to the limit
    pub fn limit_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/limit")
    }

    /// Get the maximum number of orbitals that can be sold
    pub fn limit(&self) -> u128 {
        self.limit_pointer().get_value::<u128>()
    }

    /// Set the maximum number of orbitals that can be sold
    pub fn set_limit(&self, limit: u128) {
        self.limit_pointer().set_value::<u128>(if limit == 0 { u128::MAX } else { limit });
    }

    /// Get the pointer to the sold counter
    pub fn sold_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/sold")
    }

    /// Get the number of orbitals sold
    pub fn sold(&self) -> u128 {
        self.sold_pointer().get_value::<u128>()
    }

    /// Set the number of orbitals sold
    pub fn set_sold(&self, sold: u128) {
        self.sold_pointer().set_value::<u128>(sold);
    }

    /// Increment the sold counter
    pub fn increment_sold(&self, count: u128) -> Result<()> {
        self.set_sold(overflow_error(self.sold().checked_add(count))
            .map_err(|_| anyhow!("sold counter overflow"))?);
        Ok(())
    }

    /// Get the terms of service
    pub fn terms_of_service(&self) -> String {
        "TERMS OF SERVICE AND SALE\n\n\
        By using this service and purchasing non-fungible units of this digital asset with Bitcoin, you agree to the following terms:\n\n\
        1. The digital assets provided are sold as-is without any warranty, express or implied.\n\
        2. The seller is not liable for any damages arising from the use of these digital assets.\n\
        3. You assume all risks associated with the use of these digital assets.\n\
        4. These digital assets are released without warranty to be used at your own risk.\n\
        5. The seller makes no guarantees regarding the value, utility, or functionality of these digital assets.\n\
        6. By completing a purchase, you acknowledge that you have read and agree to these terms.\n\
        7. Bitcoin payments cannot be automatically refunded. If you send more Bitcoin than needed for your purchase, any excess amount will not be refunded through this system.\n\
        8. The number of orbitals minted will be limited by the amount of Bitcoin sent and the available fuel for processing.\n\n\
        All sales are final. No refunds will be provided under any circumstances."
            .to_string()
    }

    /// Get the fuel amount for calls
    pub fn fuel(&self) -> u64 {
        // Default fuel value
        1000000
    }

    /// Observe initialization to prevent multiple initializations
    pub fn observe_initialization(&self) -> Result<()> {
        let mut pointer = StoragePointer::from_keyword("/initialized");
        if pointer.get().len() == 0 {
            pointer.set_value::<u8>(0x01);
            Ok(())
        } else {
            Err(anyhow!("already initialized"))
        }
    }

    /// Check if a transaction has already been processed
    fn observe_transaction(&self, tx: &Transaction) -> Result<()> {
        let txid = tx.compute_txid();
        
        let mut ptr = StoragePointer::from_keyword("/seen/").select(&txid.as_byte_array().to_vec());
        if ptr.get().len() != 0 {
            Err(anyhow!("transaction already processed"))
        } else {
            ptr.set_value::<u8>(0x01);
            Ok(())
        }
    }

    /// Compute the total output value sent to the taproot address
    fn compute_btc_output(&self, tx: &Transaction) -> u128 {
        let taproot_script = self.taproot_address_script();
        if taproot_script.is_empty() {
            return 0;
        }
        
        let total = tx.output.iter().fold(0, |r: u128, v: &TxOut| -> u128 {
            if v.script_pubkey.as_bytes().to_vec() == taproot_script {
                r + <u64 as Into<u128>>::into(v.value.to_sat())
            } else {
                r
            }
        });
        
        total
    }

    /// Calculate the number of orbitals that can be purchased with the given BTC amount
    pub fn calculate_purchase_count(&self, btc_amount: u128) -> u128 {
        let price = self.price();
        if price == 0 {
            return 0;
        }
        
        // Round down to ensure we don't mint more than paid for
        btc_amount / price
    }

    /// Initialize the sale
    fn initialize(
        &self,
        price: u128,
        limit: u128,
        taproot_part1: u128,
        taproot_part2: u128,
        taproot_part3: u128,
        name_part1: u128,
        name_part2: u128,
        symbol: u128
    ) -> Result<CallResponse> {
        let context = self.context()?;
        let response = CallResponse::forward(&context.incoming_alkanes);

        // Configure the Bitcoin network
        configure_network();

        // Prevent multiple initializations
        self.observe_initialization()?;

        // Get the current sequence number
        let sequence = context.myself.tx;

        // Deploy the bitcoin-collection alkane using [6, BITCOIN_COLLECTION_TEMPLATE_ID]
        let collection_cellpack = Cellpack {
            target: AlkaneId {
                block: 6,
                tx: BITCOIN_COLLECTION_TEMPLATE_ID,
            },
            inputs: vec![0, name_part1, name_part2, symbol], // Initialize opcode with name and symbol
        };
        
        let _collection_response = self.call(
            &collection_cellpack,
            &AlkaneTransferParcel::default(),
            self.fuel()
        )?;
        
        // Calculate the collection alkane ID as [2, sequence + 1]
        let collection_id = AlkaneId {
            block: 2,
            tx: sequence + 1,
        };

        // Set the collection alkane ID
        self.set_collection_alkane_id(&collection_id);

        // Set the taproot address from the three parts
        self.set_taproot_address(taproot_part1, taproot_part2, taproot_part3);

        // Set the price
        self.set_price(price);

        // Set the limit
        self.set_limit(limit);

        // Initialize the sold counter
        self.set_sold(0);

        Ok(response)
    }

    /// Purchase orbitals using BTC
    fn purchase(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::default();

        // Configure the Bitcoin network
        configure_network();

        // Check if the sale has reached its limit
        if self.sold() >= self.limit() {
            return Err(anyhow!("Sale limit reached"));
        }

        // Parse the Bitcoin transaction
        let tx = consensus_decode::<Transaction>(&mut std::io::Cursor::new(self.transaction()))
            .map_err(|e| anyhow!("Failed to parse Bitcoin transaction: {}", e))?;
        let txid = tx.compute_txid();

        // Check if the transaction has already been processed
        self.observe_transaction(&tx)?;

        // Compute the amount of BTC sent to the taproot address
        let btc_amount = self.compute_btc_output(&tx);
        
        // Check if payment was provided
        if btc_amount == 0 {
            return Err(anyhow!("No BTC payment sent to the specified taproot address"));
        }

        // Calculate how many orbitals can be purchased
        let purchase_count = self.calculate_purchase_count(btc_amount);
        
        // Check if at least one orbital can be purchased
        if purchase_count == 0 {
            return Err(anyhow!("Insufficient BTC payment"));
        }

        // Check if the purchase would exceed the limit
        let new_sold = self.sold() + purchase_count;
        let actual_purchase_count = if new_sold > self.limit() {
            self.limit() - self.sold()
        } else {
            purchase_count
        };

        if actual_purchase_count == 0 {
            return Err(anyhow!("Sale limit reached"));
        }

        // Get the collection alkane ID
        let collection_id = self.collection_alkane_id()?;

        // Create a vector to store the purchased orbitals
        let mut purchased_orbitals = Vec::new();
        let mut minted_count = 0u128;

        // Purchase the orbitals, checking fuel before each mint
        for _ in 0..actual_purchase_count {
            // Check if we have enough fuel for this mint
            if <Self as AlkaneResponder>::fuel(&self) < 500000 {  // Minimum fuel needed for minting
                break;
            }

            // Call the collection's CreateOrbital opcode
            let cellpack = Cellpack {
                target: collection_id,
                inputs: vec![77], // CreateOrbital opcode
            };
            
            let orbital_response = match self.call(
                &cellpack,
                &AlkaneTransferParcel::default(),
                self.fuel()
            ) {
                Ok(response) => response,
                Err(e) => return Err(anyhow!("Error minting orbital: {}", e)),
            };
            
            // Extract the orbital instance ID from the response
            // The response data format is: [block(16 bytes)][tx(16 bytes)][index(16 bytes)]
            if orbital_response.data.len() < 48 {
                return Err(anyhow!("Invalid response from collection"));
            }
            
            let orbital_id = AlkaneId {
                block: u128::from_le_bytes(orbital_response.data[0..16].try_into()
                    .map_err(|_| anyhow!("Failed to parse orbital block ID"))?),
                tx: u128::from_le_bytes(orbital_response.data[16..32].try_into()
                    .map_err(|_| anyhow!("Failed to parse orbital tx ID"))?),
            };
            
            // Add the orbital to the purchased orbitals
            purchased_orbitals.push(AlkaneTransfer {
                id: orbital_id,
                value: 1u128,
            });
            
            minted_count += 1;
        }

        // Update the sold counter with the actual number minted
        self.increment_sold(minted_count)?;

        // Add the purchased orbitals to the response
        response.alkanes.0.extend(purchased_orbitals);

        // Include information about the purchase in the response data
        let mut info = format!(
            "Successfully minted {} orbitals. BTC payment: {} satoshis. Transaction ID: {}",
            minted_count, btc_amount, txid
        ).into_bytes();
        
        // If we couldn't mint all requested orbitals, include that information
        if minted_count < actual_purchase_count {
            let additional_info = format!(
                "\nCould only mint {} out of {} requested orbitals due to fuel limitations.",
                minted_count, actual_purchase_count
            ).into_bytes();
            info.extend(additional_info);
        }
        
        response.data = info;

        Ok(response)
    }

    /// Get the collection alkane ID
    fn get_collection_alkane_id(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        // Get the collection alkane ID
        let collection_id = self.collection_alkane_id()?;
        
        // Serialize the AlkaneId to bytes
        let mut bytes = Vec::with_capacity(32);
        bytes.extend_from_slice(&collection_id.block.to_le_bytes());
        bytes.extend_from_slice(&collection_id.tx.to_le_bytes());
        
        response.data = bytes;

        Ok(response)
    }

    /// Get the price per orbital
    fn get_price(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        response.data = self.price().to_le_bytes().to_vec();

        Ok(response)
    }

    /// Get the maximum number of orbitals that can be sold
    fn get_limit(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        response.data = self.limit().to_le_bytes().to_vec();

        Ok(response)
    }

    /// Get the number of orbitals sold
    fn get_sold(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        response.data = self.sold().to_le_bytes().to_vec();

        Ok(response)
    }

    /// Get the terms of service
    fn get_terms_of_service(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        response.data = self.terms_of_service().into_bytes();

        Ok(response)
    }

    /// Get the taproot address
    fn get_taproot_address(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        response.data = self.taproot_address().into_bytes();

        Ok(response)
    }
    
    /// Get the beneficiary address (view function)
    fn get_beneficiary(&self) -> Result<CallResponse> {
        let _context = self.context()?;
        let mut response = CallResponse::default();
        
        // Configure the network to ensure proper address formatting
        configure_network();
        
        // Get the beneficiary address
        response.data = self.taproot_address().into_bytes();
        
        Ok(response)
    }
}

impl AlkaneResponder for BitcoinSale {
    fn execute(&self) -> Result<CallResponse> {
        // This method should not be called directly when using MessageDispatch
        Err(anyhow!("This method should not be called directly. Use the declare_alkane macro instead."))
    }
}

// Use the declare_alkane macro
declare_alkane! {
    impl AlkaneResponder for BitcoinSale {
        type Message = BitcoinSaleMessage;
    }
}