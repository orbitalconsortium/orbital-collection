use alkanes_runtime::declare_alkane;
use alkanes_runtime::message::MessageDispatch;
#[allow(unused_imports)]
use alkanes_runtime::{
    println,
    stdio::{stdout, Write},
};
use alkanes_runtime::{runtime::AlkaneResponder, storage::StoragePointer, token::Token};
use alkanes_support::{parcel::AlkaneTransfer, response::CallResponse};
use anyhow::{anyhow, Result};
use metashrew_support::compat::{to_arraybuffer_layout, to_passback_ptr};
use metashrew_support::index_pointer::KeyValuePointer;
use alkanes_support::context::Context;
use alkanes_support::utils::overflow_error;
use alkanes_support::id::AlkaneId;
use alkanes_support::parcel::AlkaneTransferParcel;
use alkanes_support::cellpack::Cellpack;
use std::sync::Arc;
use std::io::Cursor;

/// Sale alkane for selling orbital instances
#[derive(Default)]
pub struct Sale(());

/// Message enum for opcode-based dispatch
#[derive(MessageDispatch)]
enum SaleMessage {
    /// Initialize the sale
    #[opcode(0)]
    Initialize {
        /// Collection alkane block
        collection_alkane_block: u128,
        /// Collection alkane tx
        collection_alkane_tx: u128,
        /// Payment alkane block
        payment_alkane_block: u128,
        /// Payment alkane tx
        payment_alkane_tx: u128,
        /// Price per orbital
        price: u128,
        /// Maximum number of orbitals that can be sold
        limit: u128,
    },

    /// Purchase an orbital
    #[opcode(77)]
    Purchase,

    /// Get the collection alkane ID
    #[opcode(99)]
    #[returns(Vec<u8>)]
    GetCollectionAlkaneId,

    /// Get the payment alkane ID
    #[opcode(100)]
    #[returns(Vec<u8>)]
    GetPaymentAlkaneId,

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
}

impl Sale {
    /// Get the pointer to the collection alkane ID
    pub fn collection_alkane_id_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/collection-alkane-id")
    }

    /// Get the collection alkane ID
    pub fn collection_alkane_id(&self) -> AlkaneId {
        let data = self.collection_alkane_id_pointer().get();
        if data.len() == 0 {
            // This should never happen if initialized properly
            panic!("Collection alkane ID not found");
        }
        
        // Deserialize the AlkaneId from storage
        let bytes = data.as_ref();
        AlkaneId {
            block: u128::from_le_bytes(bytes[0..16].try_into().unwrap()),
            tx: u128::from_le_bytes(bytes[16..32].try_into().unwrap()),
        }
    }

    /// Set the collection alkane ID
    pub fn set_collection_alkane_id(&self, id: &AlkaneId) {
        // Serialize the AlkaneId to bytes
        let mut bytes = Vec::with_capacity(32);
        bytes.extend_from_slice(&id.block.to_le_bytes());
        bytes.extend_from_slice(&id.tx.to_le_bytes());
        
        self.collection_alkane_id_pointer().set(Arc::new(bytes));
    }

    /// Get the pointer to the payment alkane ID
    pub fn payment_alkane_id_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/payment-alkane-id")
    }

    /// Get the payment alkane ID
    pub fn payment_alkane_id(&self) -> AlkaneId {
        let data = self.payment_alkane_id_pointer().get();
        if data.len() == 0 {
            // This should never happen if initialized properly
            panic!("Payment alkane ID not found");
        }
        
        // Deserialize the AlkaneId from storage
        let bytes = data.as_ref();
        AlkaneId {
            block: u128::from_le_bytes(bytes[0..16].try_into().unwrap()),
            tx: u128::from_le_bytes(bytes[16..32].try_into().unwrap()),
        }
    }

    /// Set the payment alkane ID
    pub fn set_payment_alkane_id(&self, id: &AlkaneId) {
        // Serialize the AlkaneId to bytes
        let mut bytes = Vec::with_capacity(32);
        bytes.extend_from_slice(&id.block.to_le_bytes());
        bytes.extend_from_slice(&id.tx.to_le_bytes());
        
        self.payment_alkane_id_pointer().set(Arc::new(bytes));
    }

    /// Get the pointer to the price
    pub fn price_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/price")
    }

    /// Get the price per orbital
    pub fn price(&self) -> u128 {
        self.price_pointer().get_value::<u128>()
    }

    /// Set the price per orbital
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
        By using this service and purchasing non-fungible units of this digital asset, you agree to the following terms:\n\n\
        1. The digital assets provided are sold as-is without any warranty, express or implied.\n\
        2. The seller is not liable for any damages arising from the use of these digital assets.\n\
        3. You assume all risks associated with the use of these digital assets.\n\
        4. These digital assets are released without warranty to be used at your own risk.\n\
        5. The seller makes no guarantees regarding the value, utility, or functionality of these digital assets.\n\
        6. By completing a purchase, you acknowledge that you have read and agree to these terms.\n\n\
        All sales are final. No refunds will be provided under any circumstances."
            .to_string()
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

    /// Calculate the number of orbitals that can be purchased with the given payment amount
    pub fn calculate_purchase_count(&self, payment_amount: u128) -> (u128, u128) {
        let price = self.price();
        if price == 0 {
            return (0, payment_amount);
        }
        
        let count = payment_amount / price;
        let change = payment_amount % price;
        
        (count, change)
    }

    /// Initialize the sale
    fn initialize(
        &self,
        collection_alkane_block: u128,
        collection_alkane_tx: u128,
        payment_alkane_block: u128,
        payment_alkane_tx: u128,
        price: u128,
        limit: u128
    ) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        // Prevent multiple initializations
        self.observe_initialization()?;

        // Create the collection alkane ID
        let collection_id = AlkaneId {
            block: collection_alkane_block,
            tx: collection_alkane_tx,
        };

        // Create the payment alkane ID
        let payment_id = AlkaneId {
            block: payment_alkane_block,
            tx: payment_alkane_tx,
        };

        // Set the collection alkane ID
        self.set_collection_alkane_id(&collection_id);

        // Set the payment alkane ID
        self.set_payment_alkane_id(&payment_id);

        // Set the price
        self.set_price(price);

        // Set the limit
        self.set_limit(limit);

        // Initialize the sold counter
        self.set_sold(0);

        Ok(response)
    }

    /// Purchase an orbital
    fn purchase(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::default();

        // Check if the sale has reached its limit
        if self.sold() >= self.limit() {
            return Err(anyhow!("Sale limit reached"));
        }

        // Get the payment alkane ID
        let payment_id = self.payment_alkane_id();

        // Find the payment in the incoming alkanes
        let mut payment_amount = 0u128;
        for transfer in &context.incoming_alkanes.0 {
            if transfer.id == payment_id {
                payment_amount = transfer.value;
                break;
            }
        }

        // Check if payment was provided
        if payment_amount == 0 {
            return Err(anyhow!("No payment provided"));
        }

        // Calculate how many orbitals can be purchased and the change
        let (purchase_count, change) = self.calculate_purchase_count(payment_amount);
        
        // Check if at least one orbital can be purchased
        if purchase_count == 0 {
            return Err(anyhow!("Insufficient payment"));
        }

        // Check if the purchase would exceed the limit
        let new_sold = self.sold() + purchase_count;
        if new_sold > self.limit() {
            return Err(anyhow!("Purchase would exceed sale limit"));
        }

        // Create a vector to store the purchased orbitals
        let mut purchased_orbitals = Vec::new();

        // Purchase the orbitals
        for _ in 0..purchase_count {
            // Call the collection's CreateOrbital opcode
            let collection_id = self.collection_alkane_id();
            let cellpack = Cellpack {
                target: collection_id,
                inputs: vec![77], // CreateOrbital opcode
            };
            
            let orbital_response = self.call(
                &cellpack,
                &AlkaneTransferParcel::default(),
                self.fuel()?
            )?;
            
            // Extract the orbital instance ID from the response
            // The response data format is: [block(16 bytes)][tx(16 bytes)][index(16 bytes)]
            if orbital_response.data.len() < 48 {
                return Err(anyhow!("Invalid response from collection"));
            }
            
            let orbital_id = AlkaneId {
                block: u128::from_le_bytes(orbital_response.data[0..16].try_into().unwrap()),
                tx: u128::from_le_bytes(orbital_response.data[16..32].try_into().unwrap()),
            };
            
            // Add the orbital to the purchased orbitals
            purchased_orbitals.push(AlkaneTransfer {
                id: orbital_id,
                value: 1u128,
            });
        }

        // Update the sold counter
        self.increment_sold(purchase_count)?;

        // Add the purchased orbitals to the response
        response.alkanes.0.extend(purchased_orbitals);

        // Add change if any
        if change > 0 {
            response.alkanes.0.push(AlkaneTransfer {
                id: payment_id,
                value: change,
            });
        }

        Ok(response)
    }

    /// Get the collection alkane ID
    fn get_collection_alkane_id(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        // Get the collection alkane ID
        let collection_id = self.collection_alkane_id();
        
        // Serialize the AlkaneId to bytes
        let mut bytes = Vec::with_capacity(32);
        bytes.extend_from_slice(&collection_id.block.to_le_bytes());
        bytes.extend_from_slice(&collection_id.tx.to_le_bytes());
        
        response.data = bytes;

        Ok(response)
    }

    /// Get the payment alkane ID
    fn get_payment_alkane_id(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let mut response = CallResponse::forward(&context.incoming_alkanes);

        // Get the payment alkane ID
        let payment_id = self.payment_alkane_id();
        
        // Serialize the AlkaneId to bytes
        let mut bytes = Vec::with_capacity(32);
        bytes.extend_from_slice(&payment_id.block.to_le_bytes());
        bytes.extend_from_slice(&payment_id.tx.to_le_bytes());
        
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
}

impl AlkaneResponder for Sale {
    fn execute(&self) -> Result<CallResponse> {
        // This method should not be called directly when using MessageDispatch
        Err(anyhow!("This method should not be called directly. Use the declare_alkane macro instead."))
    }
}

// Use the declare_alkane macro
declare_alkane! {
    impl AlkaneResponder for Sale {
        type Message = SaleMessage;
    }
}