use alkanes_runtime::storage::StoragePointer;
use alkanes_runtime::token::Token;
use alkanes_support::response::CallResponse;
use alkanes_support::id::AlkaneId;
use alkanes_support::context::Context;
use anyhow::{anyhow, Result};
use orbital_traits::OrbitalCollection;
use orbitals_macros::{OrbitalCollectionMessage, declare_orbital_collection};
use std::sync::Arc;
use metashrew_support::index_pointer::KeyValuePointer;
use metashrew_support::compat::to_arraybuffer_layout;

/// Orbital template ID - this is the template used for creating orbital instances
pub const ORBITAL_TEMPLATE_ID: u128 = 0xe0e2;

/// Bitcoin Collection alkane that acts as a factory for orbital instances
/// Only allows the bitcoin-sale contract to mint orbitals
#[derive(Default)]
pub struct BitcoinCollection(());

/// Message enum for opcode-based dispatch
#[derive(OrbitalCollectionMessage)]
enum BitcoinCollectionMessage {
    /// Initialize the collection
    #[opcode(0)]
    Initialize {
        /// Name part 1
        name_part1: u128,
        /// Name part 2
        name_part2: u128,
        /// Symbol
        symbol: u128,
    },

    /// Create a new orbital instance (only callable by authorized alkanes)
    #[opcode(77)]
    CreateOrbital,

    /// Get the name of the collection
    #[opcode(99)]
    #[returns(String)]
    GetName,

    /// Get the symbol of the collection
    #[opcode(100)]
    #[returns(String)]
    GetSymbol,

    /// Get the total supply of the collection
    #[opcode(101)]
    #[returns(u128)]
    GetTotalSupply,

    /// Get the count of orbitals that have been minted
    #[opcode(102)]
    #[returns(u128)]
    GetOrbitalCount,

    /// Get the data of the collection with optional transform
    #[opcode(1000)]
    #[returns(Vec<u8>)]
    GetData,
}

impl Token for BitcoinCollection {
    fn name(&self) -> String {
        let name_pointer = StoragePointer::from_keyword("/name");
        let name_bytes = name_pointer.get();
        if name_bytes.len() == 0 {
            return String::from("Bitcoin Orbital Collection");
        }
        
        String::from_utf8_lossy(name_bytes.as_ref()).to_string()
    }

    fn symbol(&self) -> String {
        let symbol_pointer = StoragePointer::from_keyword("/symbol");
        let symbol_bytes = symbol_pointer.get();
        if symbol_bytes.len() == 0 {
            return String::from("BTC-Collection");
        }
        
        String::from_utf8_lossy(symbol_bytes.as_ref()).to_string()
    }
}

impl BitcoinCollection {
    /// Get the pointer to the name
    fn name_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/name")
    }

    /// Set the name
    fn set_name(&self, name: &str) {
        self.name_pointer().set(Arc::new(name.as_bytes().to_vec()));
    }

    /// Get the pointer to the symbol
    fn symbol_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/symbol")
    }

    /// Set the symbol
    fn set_symbol(&self, symbol: &str) {
        self.symbol_pointer().set(Arc::new(symbol.as_bytes().to_vec()));
    }

    /// Get the pointer to the authorized sale alkane ID
    fn authorized_sale_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/authorized-sale")
    }

    /// Get the authorized sale alkane ID
    fn authorized_sale(&self) -> Option<AlkaneId> {
        let data = self.authorized_sale_pointer().get();
        if data.len() == 0 {
            return None;
        }
        
        let bytes = data.as_ref();
        Some(AlkaneId {
            block: u128::from_le_bytes(bytes[0..16].try_into().unwrap()),
            tx: u128::from_le_bytes(bytes[16..32].try_into().unwrap()),
        })
    }

    /// Set the authorized sale alkane ID
    fn set_authorized_sale(&self, id: &AlkaneId) {
        // Serialize the AlkaneId to bytes
        let mut bytes = Vec::with_capacity(32);
        bytes.extend_from_slice(&id.block.to_le_bytes());
        bytes.extend_from_slice(&id.tx.to_le_bytes());
        
        self.authorized_sale_pointer().set(Arc::new(bytes));
    }

    /// Get the pointer to the instances count
    fn instances_count_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/instances-count")
    }

    /// Get the number of instances
    fn instances_count(&self) -> u128 {
        self.instances_count_pointer().get_value::<u128>()
    }

    /// Set the number of instances
    fn set_instances_count(&self, count: u128) {
        self.instances_count_pointer().set_value::<u128>(count);
    }

    /// Increment the instances count
    fn increment_instances_count(&self) -> u128 {
        let count = self.instances_count();
        let new_count = count + 1;
        self.set_instances_count(new_count);
        new_count
    }

    /// Observe initialization to prevent multiple initializations
    fn observe_initialization(&self) -> Result<()> {
        let mut pointer = StoragePointer::from_keyword("/initialized");
        if pointer.get().len() == 0 {
            pointer.set_value::<u8>(0x01);
            Ok(())
        } else {
            Err(anyhow!("already initialized"))
        }
    }

    /// Get the context for the current execution
    fn context(&self) -> Result<Context> {
        Ok(Context::default())
    }
}

impl OrbitalCollection for BitcoinCollection {
    /// Get the orbital template ID
    fn orbital_template(&self) -> u128 {
        ORBITAL_TEMPLATE_ID
    }
    
    /// Check if an alkane ID is authorized to create orbitals
    /// Only the stored sale alkane is authorized
    fn is_authorized(&self, alkane_id: &AlkaneId) -> bool {
        if let Some(authorized_sale) = self.authorized_sale() {
            *alkane_id == authorized_sale
        } else {
            false
        }
    }
    
    /// Initialize the collection
    fn initialize(&self, name_part1: u128, name_part2: u128, symbol: u128) -> Result<CallResponse> {
        let context = self.context()?;
        let response = CallResponse::forward(&context.incoming_alkanes);

        // Prevent multiple initializations
        self.observe_initialization()?;

        // Set the name
        let name = format!("{}{}", 
            String::from_utf8_lossy(&name_part1.to_le_bytes()).trim_matches(char::from(0)),
            String::from_utf8_lossy(&name_part2.to_le_bytes()).trim_matches(char::from(0))
        );
        self.set_name(&name);

        // Set the symbol
        let symbol_str = String::from_utf8_lossy(&symbol.to_le_bytes()).trim_matches(char::from(0)).to_string();
        self.set_symbol(&symbol_str);

        // Initialize the instances count
        self.set_instances_count(0);

        // Store the caller (bitcoin-sale) as the authorized alkane
        self.set_authorized_sale(&context.caller);

        Ok(response)
    }
    
    /// Create a new orbital instance
    fn create_orbital(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        // Check if the caller is authorized
        if !self.is_authorized(&context.caller) {
            return Err(anyhow!("Unauthorized caller"));
        }

        // Increment the instances count
        let index = self.increment_instances_count();

        // In a real implementation, we would create the orbital instance here
        // For now, we'll just return the index
        response.data = index.to_le_bytes().to_vec();

        Ok(response)
    }
    
    /// Get the name of the collection
    fn get_name(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        response.data = self.name().into_bytes();

        Ok(response)
    }
    
    /// Get the symbol of the collection
    fn get_symbol(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        response.data = self.symbol().into_bytes();

        Ok(response)
    }
    
    /// Get the total supply of the collection
    fn get_total_supply(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        // Collection itself doesn't have units, so always return 0
        response.data = 0u128.to_le_bytes().to_vec();

        Ok(response)
    }
    
    /// Get the count of orbitals that have been minted
    fn get_orbital_count(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        response.data = self.instances_count().to_le_bytes().to_vec();

        Ok(response)
    }
    
    /// Get the data of the collection
    fn get_data(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        // In a real implementation, we would get the data from the container
        // For now, we'll just return an empty vector
        response.data = Vec::new();

        Ok(response)
    }
}

// Use the declare_orbital_collection macro
declare_orbital_collection! {
    impl AlkaneResponder for BitcoinCollection {
        type Message = BitcoinCollectionMessage;
    }
}