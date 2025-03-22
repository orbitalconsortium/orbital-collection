use alkanes_runtime::{runtime::AlkaneResponder, storage::StoragePointer, token::Token};
use alkanes_support::{parcel::AlkaneTransferParcel, response::CallResponse, id::AlkaneId, cellpack::Cellpack};
use anyhow::{anyhow, Result};
use std::sync::Arc;

// Example implementations of BytesTransform
pub mod examples;

// Example of a custom orbital implementation
pub mod custom_orbital_example;

/// Trait for transforming data bytes
pub trait BytesTransform: Send + Sync {
    /// Transform the input bytes based on the index and sequence
    fn transform(&self, input: &[u8], index: u128, sequence: u128) -> Vec<u8>;
}

/// A transform that passes the bytes through without modification
pub struct IdentityTransform;

impl BytesTransform for IdentityTransform {
    fn transform(&self, input: &[u8], _index: u128, _sequence: u128) -> Vec<u8> {
        input.to_vec()
    }
}

/// Trait for orbital alkanes
pub trait Orbital: AlkaneResponder + Token {
    /// Get the pointer to the collection alkane ID
    fn collection_alkane_id_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/collection-alkane-id")
    }

    /// Get the collection alkane ID reference
    fn collection_ref(&self) -> AlkaneId {
        let data = self.collection_alkane_id_pointer().get();
        if data.len() == 0 {
            // This should never happen if initialized properly
            panic!("Collection reference not found");
        }
        
        // Deserialize the AlkaneId from storage
        let bytes = data.as_ref();
        AlkaneId {
            block: u128::from_le_bytes(bytes[0..16].try_into().unwrap()),
            tx: u128::from_le_bytes(bytes[16..32].try_into().unwrap()),
        }
    }

    /// Set the collection alkane ID
    fn set_collection_alkane_id(&self, id: &AlkaneId) {
        // Serialize the AlkaneId to bytes
        let mut bytes = Vec::with_capacity(32);
        bytes.extend_from_slice(&id.block.to_le_bytes());
        bytes.extend_from_slice(&id.tx.to_le_bytes());
        
        self.collection_alkane_id_pointer().set(Arc::new(bytes));
    }

    /// Get the pointer to the index
    fn index_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/index")
    }

    /// Get the index of this orbital in the collection (0-based)
    fn index(&self) -> u128 {
        self.index_pointer().get_value::<u128>()
    }

    /// Set the index of this orbital in the collection
    fn set_index(&self, index: u128) {
        self.index_pointer().set_value::<u128>(index);
    }

    /// Get the sequence number of this orbital
    fn sequence(&self) -> u128 {
        // The sequence number is the tx part of our AlkaneId
        match self.context() {
            Ok(context) => context.myself.tx,
            Err(_) => 0, // This should never happen
        }
    }

    /// Get the collection's name using staticcall
    fn get_collection_name(&self) -> Result<String> {
        // Create a cellpack to call the collection's GetName opcode
        let cellpack = Cellpack {
            target: self.collection_ref(),
            inputs: vec![99], // GetName opcode
        };
        
        // Call the collection's GetName opcode
        let call_response = self.staticcall(
            &cellpack,
            &AlkaneTransferParcel::default(),
            self.fuel()?
        )?;
        
        // Convert the response data to a string
        String::from_utf8(call_response.data).map_err(|e| anyhow!("Invalid UTF-8: {}", e))
    }

    /// Get the collection's symbol using staticcall
    fn get_collection_symbol(&self) -> Result<String> {
        // Create a cellpack to call the collection's GetSymbol opcode
        let cellpack = Cellpack {
            target: self.collection_ref(),
            inputs: vec![100], // GetSymbol opcode
        };
        
        // Call the collection's GetSymbol opcode
        let call_response = self.staticcall(
            &cellpack,
            &AlkaneTransferParcel::default(),
            self.fuel()?
        )?;
        
        // Convert the response data to a string
        String::from_utf8(call_response.data).map_err(|e| anyhow!("Invalid UTF-8: {}", e))
    }

    /// Convert a number to its superscript representation
    fn to_superscript(&self, num: u128) -> String {
        if num == 0 {
            return "⁰".to_string();
        }
        
        let mut result = String::new();
        let mut n = num;
        
        while n > 0 {
            let digit = (n % 10) as u8;
            let superscript = match digit {
                0 => "⁰",
                1 => "¹",
                2 => "²",
                3 => "³",
                4 => "⁴",
                5 => "⁵",
                6 => "⁶",
                7 => "⁷",
                8 => "⁸",
                9 => "⁹",
                _ => unreachable!(),
            };
            result.insert_str(0, superscript);
            n /= 10;
        }
        
        result
    }

    /// Get the pointer to the total supply
    fn total_supply_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/totalsupply")
    }

    /// Get the total supply
    fn total_supply(&self) -> u128 {
        self.total_supply_pointer().get_value::<u128>()
    }

    /// Set the total supply
    fn set_total_supply(&self, v: u128) {
        self.total_supply_pointer().set_value::<u128>(v);
    }

    /// Observe initialization to prevent multiple initializations
    fn observe_initialization(&self) -> Result<()> {
        let mut initialized_pointer = StoragePointer::from_keyword("/initialized");
        if initialized_pointer.get().len() == 0 {
            initialized_pointer.set_value::<u32>(1);
            Ok(())
        } else {
            Err(anyhow!("already initialized"))
        }
    }

    /// Get the data transform to apply
    fn get_transform(&self) -> Box<dyn BytesTransform>;

    /// Get the data of the orbital (proxies to collection with transform)
    fn get_data(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        // Get the collection alkane ID
        let collection_id = self.collection_ref();
        
        // Create a cellpack to call the collection's GetData opcode
        let cellpack = Cellpack {
            target: collection_id,
            inputs: vec![1000, self.sequence()], // GetData opcode with sequence number
        };
        
        // Call the collection's GetData opcode
        let call_response = self.staticcall(
            &cellpack,
            &AlkaneTransferParcel::default(),
            self.fuel()?
        )?;
        
        // Get the transform
        let transform = self.get_transform();
        
        // Apply the transform to the data
        let transformed_data = transform.transform(
            &call_response.data,
            self.index(),
            self.sequence()
        );
        
        // Return the transformed data
        response.data = transformed_data;

        Ok(response)
    }

    /// Default implementation for name
    fn default_name(&self) -> String {
        // Get the collection's name and add the superscript index
        match self.get_collection_name() {
            Ok(name) => {
                let index = self.index();
                format!("{}{}", name, self.to_superscript(index))
            },
            Err(_) => {
                // Fallback if we can't get the collection's name
                format!("Orbital #{}", self.index())
            }
        }
    }
    
    /// Default implementation for symbol
    fn default_symbol(&self) -> String {
        // Get the collection's symbol and add the superscript index
        match self.get_collection_symbol() {
            Ok(symbol) => {
                let index = self.index();
                format!("{}{}", symbol, self.to_superscript(index))
            },
            Err(_) => {
                // Fallback if we can't get the collection's symbol
                format!("ORB{}", self.index())
            }
        }
    }
}