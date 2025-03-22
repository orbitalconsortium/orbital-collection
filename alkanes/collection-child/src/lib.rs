use alkanes_runtime::declare_alkane;
use alkanes_runtime::message::MessageDispatch;
#[allow(unused_imports)]
use alkanes_runtime::{
    println,
    stdio::{stdout, Write},
};
use alkanes_runtime::{runtime::AlkaneResponder, token::Token};
use alkanes_support::{parcel::AlkaneTransfer, response::CallResponse};
use anyhow::{anyhow, Result};
use metashrew_support::compat::{to_arraybuffer_layout, to_passback_ptr};
use orbitals_support::{Orbital, BytesTransform, IdentityTransform};
use std::sync::Arc;

/// Orbital alkane that represents an instance of the collection
#[derive(Default)]
pub struct OrbitalInstance(());

#[derive(MessageDispatch)]
enum OrbitalMessage {
    /// Initialize the orbital with its index in the collection
    #[opcode(0)]
    Initialize {
        /// Index in the collection (0-based)
        index: u128,
    },

    /// Get the name of the orbital
    #[opcode(99)]
    #[returns(String)]
    GetName,

    /// Get the symbol of the orbital
    #[opcode(100)]
    #[returns(String)]
    GetSymbol,

    /// Get the total supply of the orbital
    #[opcode(101)]
    #[returns(u128)]
    GetTotalSupply,

    /// Get the data of the orbital (proxies to collection with transform)
    #[opcode(1000)]
    #[returns(Vec<u8>)]
    GetData,
}

impl Token for OrbitalInstance {
    fn name(&self) -> String {
        self.default_name()
    }
    
    fn symbol(&self) -> String {
        self.default_symbol()
    }
}

impl Orbital for OrbitalInstance {
    fn get_transform(&self) -> Box<dyn BytesTransform> {
        // Use the identity transform by default
        Box::new(IdentityTransform)
    }
}

impl OrbitalInstance {
    /// Initialize the orbital with its index in the collection
    fn initialize(&self, index: u128) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        self.observe_initialization()?;
        
        // Store the caller's AlkaneId (the collection that created us)
        self.set_collection_alkane_id(&context.caller);
        
        // Store the index in the collection
        self.set_index(index);
        
        // Set the total supply to 1 (each orbital is unique)
        self.set_total_supply(1);
        
        // Mint the orbital token
        response.alkanes.0.push(AlkaneTransfer {
            id: context.myself.clone(),
            value: 1u128,
        });

        Ok(response)
    }

    /// Get the name of the orbital
    fn get_name(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        response.data = self.name().into_bytes().to_vec();

        Ok(response)
    }

    /// Get the symbol of the orbital
    fn get_symbol(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        response.data = self.symbol().into_bytes().to_vec();

        Ok(response)
    }

    /// Get the total supply of the orbital
    fn get_total_supply(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        response.data = (&self.total_supply().to_le_bytes()).to_vec();

        Ok(response)
    }

    /// Get the data of the orbital (proxies to collection with transform)
    fn get_data(&self) -> Result<CallResponse> {
        // Use the implementation from the Orbital trait
        Orbital::get_data(self)
    }
}

impl AlkaneResponder for OrbitalInstance {
    fn execute(&self) -> Result<CallResponse> {
        // The opcode extraction and dispatch logic is now handled by the declare_alkane macro
        // This method is still required by the AlkaneResponder trait, but we can just return an error
        // indicating that it should not be called directly
        Err(anyhow!(
            "This method should not be called directly. Use the declare_alkane macro instead."
        ))
    }
}

// Use the new macro format
declare_alkane! {
    impl AlkaneResponder for OrbitalInstance {
        type Message = OrbitalMessage;
    }
}
