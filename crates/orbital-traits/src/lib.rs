use alkanes_support::id::AlkaneId;
use alkanes_support::response::CallResponse;
use anyhow::Result;
use orbitals_support::BytesTransform;

/// A trait for customizable orbital collections
pub trait OrbitalCollection {
    /// Get the orbital template ID
    fn orbital_template(&self) -> u128;
    
    /// Check if an alkane ID is authorized to create orbitals
    fn is_authorized(&self, alkane_id: &AlkaneId) -> bool;
    
    /// Initialize the collection
    fn initialize(&self, name_part1: u128, name_part2: u128, symbol: u128) -> Result<CallResponse>;
    
    /// Create a new orbital instance
    fn create_orbital(&self) -> Result<CallResponse>;
    
    /// Get the name of the collection
    fn get_name(&self) -> Result<CallResponse>;
    
    /// Get the symbol of the collection
    fn get_symbol(&self) -> Result<CallResponse>;
    
    /// Get the total supply of the collection
    fn get_total_supply(&self) -> Result<CallResponse>;
    
    /// Get the count of orbitals that have been minted
    fn get_orbital_count(&self) -> Result<CallResponse>;
    
    /// Get the data of the collection
    fn get_data(&self) -> Result<CallResponse>;
}

/// A trait for customizable orbitals
pub trait OrbitalInstance {
    /// Get the transform to apply to the data
    fn get_transform(&self) -> Box<dyn BytesTransform>;
    
    /// Initialize the orbital instance
    fn initialize(&self, index: u128) -> Result<CallResponse>;
    
    /// Get the default name for the orbital
    fn get_name(&self) -> Result<CallResponse>;
    
    /// Get the default symbol for the orbital
    fn get_symbol(&self) -> Result<CallResponse>;
    
    /// Get the total supply of the orbital
    fn get_total_supply(&self) -> Result<CallResponse>;
    
    /// Get the data of the orbital
    fn get_data(&self) -> Result<CallResponse>;
}

/// Message opcodes for orbital collections
pub mod collection_opcodes {
    pub const INITIALIZE: u128 = 0;
    pub const CREATE_ORBITAL: u128 = 77;
    pub const GET_NAME: u128 = 99;
    pub const GET_SYMBOL: u128 = 100;
    pub const GET_TOTAL_SUPPLY: u128 = 101;
    pub const GET_ORBITAL_COUNT: u128 = 102;
    pub const GET_DATA: u128 = 1000;
}

/// Message opcodes for orbital instances
pub mod orbital_opcodes {
    pub const INITIALIZE: u128 = 0;
    pub const GET_NAME: u128 = 99;
    pub const GET_SYMBOL: u128 = 100;
    pub const GET_TOTAL_SUPPLY: u128 = 101;
    pub const GET_DATA: u128 = 1000;
}