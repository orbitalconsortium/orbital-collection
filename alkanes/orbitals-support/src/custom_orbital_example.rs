use alkanes_runtime::declare_alkane;
use alkanes_runtime::message::MessageDispatch;
use alkanes_runtime::{runtime::AlkaneResponder, token::Token};
use alkanes_support::{parcel::AlkaneTransfer, response::CallResponse};
use anyhow::{anyhow, Result};
use crate::{Orbital, BytesTransform};
use metashrew_support::compat::to_arraybuffer_layout;

/// Example of a custom transform that could be used with image libraries
pub struct CustomImageTransform;

impl BytesTransform for CustomImageTransform {
    fn transform(&self, input: &[u8], _index: u128, _sequence: u128) -> Vec<u8> {
        // This is a placeholder implementation
        // In a real implementation, you would:
        // 1. Parse the input bytes as an image (e.g., using the image crate)
        // 2. Apply transformations based on the index and sequence
        // 3. Encode the transformed image back to bytes
        
        // For example, with the image crate:
        // use image::{ImageBuffer, Rgba};
        // 
        // // Parse the input bytes as an image
        // let img = image::load_from_memory(input).unwrap();
        // 
        // // Apply transformations based on the index and sequence
        // let mut transformed = img.clone();
        // 
        // // Example: Apply a unique transformation based on the index
        // match index % 4 {
        //     0 => transformed = transformed.grayscale(),
        //     1 => transformed = transformed.rotate90(),
        //     2 => transformed = transformed.fliph(),
        //     3 => transformed = transformed.flipv(),
        //     _ => unreachable!(),
        // }
        // 
        // // Encode the transformed image back to bytes
        // let mut buffer = Vec::new();
        // transformed.write_to(&mut buffer, image::ImageFormat::Png).unwrap();
        // buffer

        // For now, just return the input bytes unchanged
        input.to_vec()
    }
}

/// Example of a custom orbital implementation that uses a custom transform
#[derive(Default)]
pub struct CustomOrbital(());

#[derive(MessageDispatch)]
enum CustomOrbitalMessage {
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

impl Token for CustomOrbital {
    fn name(&self) -> String {
        // Use the default implementation from the Orbital trait
        self.default_name()
    }
    
    fn symbol(&self) -> String {
        // Use the default implementation from the Orbital trait
        self.default_symbol()
    }
}

impl Orbital for CustomOrbital {
    fn get_transform(&self) -> Box<dyn BytesTransform> {
        // Use our custom transform
        Box::new(CustomImageTransform)
    }
}

impl CustomOrbital {
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

impl AlkaneResponder for CustomOrbital {
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
    impl AlkaneResponder for CustomOrbital {
        type Message = CustomOrbitalMessage;
    }
}