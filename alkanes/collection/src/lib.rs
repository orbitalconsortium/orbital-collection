use alkanes_runtime::declare_alkane;
use alkanes_runtime::message::MessageDispatch;
#[allow(unused_imports)]
use alkanes_runtime::{
    println,
    stdio::{stdout, Write},
};
use alkanes_runtime::{runtime::AlkaneResponder, storage::StoragePointer, token::Token};
use alkanes_support::response::CallResponse;
use anyhow::{anyhow, Result};
use metashrew_support::compat::{to_arraybuffer_layout, to_passback_ptr};
use metashrew_support::index_pointer::KeyValuePointer;
use alkanes_support::utils::overflow_error;
use alkanes_support::id::AlkaneId;
use alkanes_support::parcel::AlkaneTransferParcel;
use alkanes_support::cellpack::Cellpack;
use std::sync::Arc;
use alkanes_runtime::imports::__call;

/// Orbital template ID - this is the template used for creating orbital instances
pub const ORBITAL_TEMPLATE_ID: u128 = 0xe0e2;

/// Collection alkane that acts as a factory for orbital instances
#[derive(Default)]
pub struct Collection(());

/// TokenName struct to hold two u128 values for the name
#[derive(Default, Clone, Copy)]
pub struct TokenName {
    pub part1: u128,
    pub part2: u128,
}

impl From<TokenName> for String {
    fn from(name: TokenName) -> Self {
        // Trim both parts and concatenate them
        format!("{}{}", trim(name.part1), trim(name.part2))
    }
}

impl TokenName {
    pub fn new(part1: u128, part2: u128) -> Self {
        Self { part1, part2 }
    }
}

/// Trims a u128 value to a String by removing trailing zeros
pub fn trim(v: u128) -> String {
    String::from_utf8(
        v.to_le_bytes()
            .into_iter()
            .fold(Vec::<u8>::new(), |mut r, v| {
                if v != 0 {
                    r.push(v)
                }
                r
            }),
    )
    .unwrap_or_default()
}

/// Message enum for opcode-based dispatch
#[derive(MessageDispatch)]
enum CollectionMessage {
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

impl Token for Collection {
    fn name(&self) -> String {
        let name_pointer = StoragePointer::from_keyword("/name");
        let name_bytes = name_pointer.get();
        if name_bytes.len() == 0 {
            return String::from("Orbital Collection");
        }
        
        String::from_utf8_lossy(name_bytes.as_ref()).to_string()
    }

    fn symbol(&self) -> String {
        let symbol_pointer = StoragePointer::from_keyword("/symbol");
        let symbol_bytes = symbol_pointer.get();
        if symbol_bytes.len() == 0 {
            return String::from("Collection");
        }
        
        String::from_utf8_lossy(symbol_bytes.as_ref()).to_string()
    }
}

impl Collection {
    /// Get the pointer to the name
    pub fn name_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/name")
    }

    /// Set the name
    pub fn set_name(&self, name: &str) {
        self.name_pointer().set(Arc::new(name.as_bytes().to_vec()));
    }

    /// Get the pointer to the symbol
    pub fn symbol_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/symbol")
    }

    /// Set the symbol
    pub fn set_symbol(&self, symbol: &str) {
        self.symbol_pointer().set(Arc::new(symbol.as_bytes().to_vec()));
    }

    /// Set the name and symbol
    pub fn set_name_and_symbol(&self, name: TokenName, symbol: u128) {
        let name_string: String = name.into();
        self.set_name(&name_string);
        self.set_string_field(self.symbol_pointer(), symbol);
    }

    /// Set a string field in storage
    fn set_string_field(&self, mut pointer: StoragePointer, v: u128) {
        pointer.set(Arc::new(trim(v).as_bytes().to_vec()));
    }

    /// Get the total supply
    pub fn total_supply(&self) -> u128 {
        // Collection itself doesn't have units, so always return 0
        0
    }

    /// Get the fuel amount for calls
    pub fn fuel(&self) -> u64 {
        // Default fuel value
        1000000
    }

    /// Get the orbital template ID
    pub fn orbital_template(&self) -> u128 {
        // Return the constant template ID
        ORBITAL_TEMPLATE_ID
    }

    /// Get the pointer to the container sequence
    pub fn container_sequence_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/container-sequence")
    }

    /// Get the container sequence
    pub fn container_sequence(&self) -> u128 {
        self.container_sequence_pointer().get_value::<u128>()
    }

    /// Set the container sequence
    pub fn set_container_sequence(&self, sequence: u128) {
        self.container_sequence_pointer().set_value::<u128>(sequence);
    }

    /// Get the pointer to the instances registry
    pub fn instances_pointer(&self) -> StoragePointer {
        StoragePointer::from_keyword("/instances")
    }

    /// Get the number of instances
    pub fn instances_count(&self) -> u128 {
        self.instances_pointer().get_value::<u128>()
    }

    /// Set the number of instances
    pub fn set_instances_count(&self, count: u128) {
        self.instances_pointer().set_value::<u128>(count);
    }

    /// Add an instance to the registry
    pub fn add_instance(&self, instance_id: &AlkaneId) -> Result<u128> {
        let count = self.instances_count();
        let new_count = overflow_error(count.checked_add(1))
            .map_err(|_| anyhow!("instances count overflow"))?;
        
        // Serialize the AlkaneId to bytes
        let mut bytes = Vec::with_capacity(32);
        bytes.extend_from_slice(&instance_id.block.to_le_bytes());
        bytes.extend_from_slice(&instance_id.tx.to_le_bytes());
        // Store the instance ID with its sequence number
        let bytes_vec = new_count.to_le_bytes().to_vec();
        let mut instance_pointer = self.instances_pointer().select(&bytes_vec);
        instance_pointer.set(Arc::new(bytes));
        
        // Update the count
        self.set_instances_count(new_count);
        
        Ok(new_count)
    }

    /// Get an instance by sequence number
    pub fn get_instance(&self, sequence: u128) -> Option<AlkaneId> {
        let bytes_vec = sequence.to_le_bytes().to_vec();
        let instance_pointer = self.instances_pointer().select(&bytes_vec);
        let instance = instance_pointer.get();
        if instance.len() > 0 {
            let bytes = instance.as_ref();
            Some(AlkaneId {
                block: u128::from_le_bytes(bytes[0..16].try_into().unwrap()),
                tx: u128::from_le_bytes(bytes[16..32].try_into().unwrap()),
            })
        } else {
            None
        }
    }

    /// Check if an alkane ID is authorized to create orbitals
    pub fn is_authorized(&self, _alkane_id: &AlkaneId) -> bool {
        // In a real implementation, we would check against a list of authorized alkanes
        // For now, we'll just return true for simplicity
        // TODO: Implement proper authorization
        true
    }

    /// Special call function for container initialization that doesn't use __returndatacopy
    /// This avoids incurring a fuel cost for what could be a very large response body
    pub fn call_without_returndata(&self, cellpack: &Cellpack, outgoing_alkanes: &AlkaneTransferParcel, fuel: u64) -> Result<()> {
        // Serialize the cellpack
        let mut cellpack_buffer = to_arraybuffer_layout::<&[u8]>(&cellpack.serialize());
        
        // Serialize the outgoing alkanes
        let mut outgoing_alkanes_buffer = to_arraybuffer_layout::<&[u8]>(&outgoing_alkanes.serialize());
        
        // Serialize the storage map
        let mut storage_map_buffer = to_arraybuffer_layout::<&[u8]>(&alkanes_runtime::runtime::get_cache().serialize());
        
        // Call the __call host function directly
        let call_result = unsafe {
            __call(
                to_passback_ptr(&mut cellpack_buffer),
                to_passback_ptr(&mut outgoing_alkanes_buffer),
                to_passback_ptr(&mut storage_map_buffer),
                fuel,
            )
        };
        
        // Check the result but don't process the return data
        match call_result {
            -1 => Err(anyhow!("call errored out")),
            _ => Ok(()),
        }
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

    /// Initialize the collection
    fn initialize(&self, name_part1: u128, name_part2: u128, symbol: u128) -> Result<CallResponse> {
        let context = self.context()?;
        let response = CallResponse::forward(&context.incoming_alkanes);

        // Prevent multiple initializations
        self.observe_initialization()?;

        // Create TokenName from the two parts
        let name = TokenName::new(name_part1, name_part2);
        self.set_name_and_symbol(name, symbol);

        // Initialize the instances count
        self.set_instances_count(0);

        // Get the sequence number for the container
        let container_sequence = match self.context() {
            Ok(context) => context.myself.tx,
            Err(_) => 0, // This should never happen
        };

        // Deploy the container using [1, 0] cellpack target with [0] opcode
        // Use the special call function that doesn't use __returndatacopy
        let container_cellpack = Cellpack {
            target: AlkaneId {
                block: 1,
                tx: 0,
            },
            inputs: vec![],
        };
        
        let _ = self.call_without_returndata(
            &container_cellpack,
            &AlkaneTransferParcel::default(),
            1
        );

        // Store the container sequence number
        self.set_container_sequence(container_sequence + 1);

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

        // Get the next sequence number
        let sequence = match self.context() {
            Ok(context) => context.myself.tx,
            Err(_) => 0, // This should never happen
        };

        // Get the next index (0-based)
        let index = self.instances_count();

        // Factory up the orbital using [6, self.orbital_template()] cellpack
        let orbital_cellpack = Cellpack {
            target: AlkaneId {
                block: 6,
                tx: self.orbital_template(),
            },
            inputs: vec![0, index], // Initialize opcode with index
        };
        
        // Call the orbital template but ignore the response
        let _orbital_call_response = self.call(
            &orbital_cellpack,
            &AlkaneTransferParcel::default(),
            self.fuel()
        )?;
        
        // Extract the orbital instance ID from the response
        // In a real implementation, we would parse the response to get the ID
        // For now, we'll just use a simplified ID
        let instance_id = AlkaneId {
            block: 2, // Simplified for demonstration
            tx: sequence,
        };
        
        // Add the instance to the registry
        self.add_instance(&instance_id)?;

        // Serialize the instance ID and index
        let mut bytes = Vec::with_capacity(48);
        bytes.extend_from_slice(&instance_id.block.to_le_bytes());
        bytes.extend_from_slice(&instance_id.tx.to_le_bytes());
        bytes.extend_from_slice(&index.to_le_bytes()); // Add the index (0-based)
        
        response.data = bytes;

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

        // Create a cellpack to call the container's GetData opcode
        // The container is at AlkaneId { block: 2, tx: self.container_sequence() }
        let container_id = AlkaneId {
            block: 2,
            tx: self.container_sequence(),
        };
        
        let cellpack = Cellpack {
            target: container_id,
            inputs: vec![1000], // GetData opcode
        };
        
        // Call the container's GetData opcode
        let call_response = self.staticcall(
            &cellpack,
            &AlkaneTransferParcel::default(),
            <Self as AlkaneResponder>::fuel(&self)
        )?;
        
        // Pass the bytes with NO transform to the caller
        response.data = call_response.data;

        Ok(response)
    }
}

impl AlkaneResponder for Collection {
    fn execute(&self) -> Result<CallResponse> {
        // This method should not be called directly when using MessageDispatch
        Err(anyhow!("This method should not be called directly. Use the declare_alkane macro instead."))
    }
}

// Use the declare_alkane macro
declare_alkane! {
    impl AlkaneResponder for Collection {
        type Message = CollectionMessage;
    }
}
