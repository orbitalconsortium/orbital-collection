# Technical Context

## Technologies Used

### Core Technologies
- **Rust** - Primary programming language
- **TypeScript** - Used for the container alkane implementation
- **WebAssembly (WASM)** - Compilation target for the alkanes
- **Alkanes Framework** - Smart contract framework for token implementation
- **MessageDispatch** - Macro for opcode-based message dispatching
- **WAT (WebAssembly Text Format)** - Used for container optimization

### Dependencies
- **alkanes-support** - Core support library for alkane contracts
- **alkanes-runtime** - Runtime support for alkane execution
- **metashrew-support** - Support library for metashrew compatibility
- **protorune-support** - Support for protorune protocol
- **ordinals** - Ordinals protocol integration
- **anyhow** - Error handling library
- **bitcoin** - Bitcoin protocol library
- **wabt.js** - WebAssembly Binary Toolkit for JavaScript
- **orbitals-support** - Custom support library for orbital alkanes

## Development Setup

### Project Structure
```
orbital-collection/
├── alkanes/
│   ├── collection/             - Collection alkane implementation
│   │   ├── Cargo.toml         - Project manifest for collection
│   │   └── src/               - Source code for collection
│   │       └── lib.rs         - Main implementation
│   ├── collection-child/       - Orbital alkane implementation
│   │   ├── Cargo.toml         - Project manifest for orbital
│   │   └── src/               - Source code for orbital
│   │       └── lib.rs         - Main implementation
│   ├── orbital-container-asm/  - Container alkane implementation
│   │   ├── Cargo.toml         - Project manifest for container
│   │   ├── package.json       - NPM package configuration
│   │   ├── tsconfig.json      - TypeScript configuration
│   │   ├── template.wat       - WebAssembly Text Format template
│   │   ├── src/               - Source code for container
│   │   │   ├── lib.rs         - Rust implementation (legacy)
│   │   │   └── index.ts       - TypeScript implementation
│   │   └── examples/          - Example usage
│   │       ├── index.html     - Browser example
│   │       └── node-example.ts - Node.js example
│   ├── orbitals-support/       - Support library for orbital alkanes
│   │   ├── Cargo.toml         - Project manifest for support library
│   │   └── src/               - Source code for support library
│   │       ├── lib.rs         - Main implementation with traits
│   │       ├── examples.rs    - Example transform implementations
│   │       └── custom_orbital_example.rs - Example custom orbital
│   └── sale/                   - Sale alkane implementation
│       ├── Cargo.toml         - Project manifest for sale
│       └── src/               - Source code for sale
│           └── lib.rs         - Main implementation
├── memory-bank/               - Documentation and context
└── reference/                 - Reference implementations
```

### Build Configuration
The project is configured as both a cdylib (for WebAssembly compilation) and rlib (for Rust library usage):

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

## Technical Constraints

### Compatibility Requirements
- Must be compatible with the alkane ecosystem
- Must follow the standard opcode format for consistency
- Must support proper initialization sequence
- Must maintain compatibility between collection and orbital instances
- Must support payment processing in the specified alkane
- Must be deployable using the oyl CLI tool

### Performance Requirements
- Container alkane must be optimized for minimal size and maximum efficiency
- Data transforms must be efficient for large collections
- Storage operations should be optimized to minimize resource usage
- WASM binary size should be minimized, especially for the container
- Payment processing must be efficient and secure
- Container initialization must avoid fuel costs for large response bodies

### Security Requirements
- Must use proper initialization guard via observe_initialization()
- Must validate all operations for proper authentication
- Must implement proper error handling for all operations
- Must ensure proper relationships between collection and orbital instances
- Must validate payment amounts in the sale alkane
- Must enforce instance limits in both collection and sale alkanes
- Must include terms of service for legal protection

## Integration Points

### Collection to Orbital Integration
The collection alkane creates and manages orbital instances:
```rust
// Example of collection creating an orbital instance
fn create_orbital(&self) -> Result<CallResponse> {
    // Get the next index
    let index = self.instances_count();

    // Factory up the orbital using [6, self.orbital_template()] cellpack
    let orbital_cellpack = Cellpack {
        block: 6,
        tx: self.orbital_template(),
        inputs: vec![0, index], // Initialize opcode with index
    };
    
    // Call to create the orbital
    // ...
}
```

### Orbital to Collection Integration
Orbital instances proxy Data requests back to the collection:
```rust
// Example of orbital proxying Data request
fn get_data(&self) -> Result<CallResponse> {
    // Get the collection alkane ID
    let collection_id = self.collection_ref();
    
    // Create a cellpack to call the collection's GetData opcode
    let cellpack = Cellpack {
        target: collection_id,
        inputs: vec![1000, self.sequence()], // GetData opcode with sequence number
    };
    
    // Call the collection's GetData opcode
    // ...
}
```

### Collection to Container Integration
The collection retrieves base data from the container:
```rust
// Example of collection retrieving data from container
fn get_data(&self) -> Result<CallResponse> {
    // Create a cellpack to call the container's GetData opcode
    let container_id = AlkaneId {
        block: 2,
        tx: self.container_sequence(),
    };
    
    let cellpack = Cellpack {
        target: container_id,
        inputs: vec![1000], // GetData opcode
    };
    
    // Call the container's GetData opcode
    // ...
}
```

### Sale to Collection Integration
The sale alkane interacts with the collection to create new instances:
```rust
// Example of sale alkane creating a new orbital instance
fn purchase(&self) -> Result<CallResponse> {
    // Verify payment
    // ...
    
    // Call the collection's CreateOrbital opcode
    let collection_id = self.collection_alkane_id();
    let cellpack = Cellpack {
        target: collection_id,
        inputs: vec![77], // CreateOrbital opcode
    };
    
    // Call to create the orbital
    // ...
}
```

### BytesTransform Interface
The orbitals-support crate provides a BytesTransform trait:
```rust
// Example of BytesTransform trait
pub trait BytesTransform: Send + Sync {
    fn transform(&self, input: &[u8], index: u128, sequence: u128) -> Vec<u8>;
}
```

### Orbital Interface
The orbitals-support crate provides an Orbital trait:
```rust
// Example of Orbital trait
pub trait Orbital: AlkaneResponder + Token {
    // Default implementations for common functionality
    // ...
    
    // Required method
    fn get_transform(&self) -> Box<dyn BytesTransform>;
}
```

### Opcode Interface
The system exposes a standardized opcode interface:
- 0: Initialize() - Initialize the alkane
- 77: Purchase() - Purchase an orbital instance (sale alkane only)
- 99: GetName() -> String
- 100: GetSymbol() -> String
- 101: GetTotalSupply() -> u128
- 102: GetOrbitalCount() -> u128 (collection alkane only)
- 104: GetTermsOfService() -> String (sale alkane only)
- 1000: GetData() -> Vec<u8>

### Storage Interface
The system uses StoragePointer for persistent state management:
- Key-value storage for token properties
- Registry storage for tracking instances
- Sequence number tracking for transforms
- Payment configuration in the sale alkane
- Instance limit enforcement in the sale alkane
- Container sequence tracking in the collection alkane

## Deployment Considerations
- The container WASM is attached to the collection deployment
- The collection is deployed using the oyl alkane new-contract command
- The orbital template is deployed separately if custom transforms are needed
- The sale alkane is deployed using the oyl alkane execute command
- Orbital instances are created through the sale alkane's purchase function
- The container alkane should be optimized for minimal size and maximum efficiency
- The sale alkane should be configured with appropriate instance limits
- The oyl CLI tool is used for all deployment operations