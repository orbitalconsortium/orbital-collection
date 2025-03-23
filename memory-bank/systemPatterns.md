# System Patterns

## Architecture Overview
The orbital-collection project implements a collection system using the alkane framework. The architecture follows a factory pattern with four main components and two support libraries:

```
orbital-collection
├── Collection Alkane (Factory)
├── Orbital Alkane (Instances)
├── Container Alkane (Data Storage)
├── Sale Alkane (Payment Processing)
├── Orbitals Support (Traits and Utilities)
└── Orbital Macros (Specialized Macros)
```

## Key Components

### Collection Alkane
The main factory implementation that:
- Creates and tracks orbital instances
- Maintains a registry of all instances it creates
- Implements the Data opcode with transform capabilities
- Proxies data requests to the container alkane
- Applies sequence-based transforms to the data
- Provides an API for the sale alkane to mint new instances
- Uses a special call function for container initialization

### Orbital Alkane
The instance implementation that:
- Implements standard token functionality
- Has a unique sequence number (address [2, n])
- Proxies Data opcode back to the collection
- Applies transforms based on its sequence number
- Maintains compatibility with the collection's factory system
- Implements the Orbital trait from orbitals-support
- Displays name and symbol with superscript index

### Container Alkane
A TypeScript library that:
- Generates minimal WASM containers
- Uses a WAT template for efficient implementation
- Provides browser and Node.js interfaces
- Efficiently embeds data in the WASM binary
- Produces the correct CallResponse structure
- Uses wabt.js for WAT to WASM conversion in the browser

### Sale Alkane
The payment processing implementation that:
- Accepts a payment alkane ID during initialization
- Sets a price for each orbital instance
- Enforces a limit on the total number of instances
- Interacts with the collection alkane to create new instances
- Handles payment processing in a single transaction
- Provides bulk purchasing with change calculation
- Includes terms of service for legal protection

### Orbitals Support
A support library that:
- Provides the BytesTransform trait for data transformations
- Provides the Orbital trait with default implementations
- Includes example implementations for custom transforms
- Offers documentation for custom transform implementation

### Orbital Macros
A specialized macro library that:
- Provides the declare_orbital! macro for orbital alkanes
- Implements the OrbitalMessage derive macro
- Generates the WebAssembly interface for orbital alkanes
- Handles MessageDispatch trait implementation
- Maintains compatibility with the enum-based opcode pattern

## Design Patterns

### Factory Pattern
The collection alkane acts as a factory for orbital instances:
- Creates new orbital instances with unique sequence numbers
- Maintains a registry of all instances it creates
- Provides a standardized interface for instance creation
- Ensures proper initialization and configuration of instances
- Exposes an API for the sale alkane to create instances

### Proxy Pattern
The Data opcode implementation uses a proxy pattern:
- Orbital instances proxy Data requests back to the collection
- The collection proxies requests to the container alkane
- Transforms are applied based on sequence numbers
- The final data is returned through the proxy chain

### Transform Pattern
The system uses the BytesTransform trait for data transformations:
- Each orbital can have a custom transform implementation
- Transforms are applied based on index and sequence numbers
- The default implementation passes data through unchanged
- Custom implementations can use image libraries or other tools
- The transform is applied after retrieving data from the collection

### Storage Pattern
The system uses StoragePointer for persistent state management:
- `/instances` - Tracks created orbital instances
- `/name` - Stores token name
- `/symbol` - Stores token symbol
- `/totalsupply` - Tracks total supply
- `/initialized` - Guards against multiple initializations
- `/payment-alkane` - Stores the payment alkane ID in the sale alkane
- `/price` - Stores the price per instance in the sale alkane
- `/limit` - Stores the maximum number of instances in the sale alkane
- `/sold` - Tracks the number of instances sold in the sale alkane
- `/container-sequence` - Stores the container sequence number in the collection
- `/index` - Stores the index of an orbital in the collection

### Sequence-Based Transform Pattern
The system applies transforms based on sequence numbers:
- Each orbital instance has a unique sequence number (address [2, n])
- The sequence number is used to deterministically transform the base data
- Transforms are applied in a consistent and predictable manner
- The base data is stored in the container alkane

### Payment Processing Pattern
The sale alkane implements a payment processing pattern:
- Accepts payment in a specified alkane (typically [2, 0])
- Verifies the payment amount matches the configured price
- Calls the collection alkane's API to create a new orbital instance
- Enforces a limit on the total number of instances that can be created
- Returns the newly created orbital instance to the purchaser
- Provides change for excess payment
- Supports bulk purchasing of multiple orbitals

### Macro Pattern
The orbital-macros crate implements specialized macros for orbital alkanes:
- The declare_orbital! macro generates WebAssembly interface functions
- The OrbitalMessage derive macro implements the MessageDispatch trait
- Macros maintain the enum-based opcode definition pattern
- Macros handle the boilerplate code for WebAssembly integration
- Macros ensure proper compatibility with the alkanes framework
- Macros provide a clean and consistent API for orbital alkanes

## Component Relationships

### Collection to Orbital Relationship
- The collection alkane creates orbital instances
- Each orbital has a reference back to its collection
- The collection maintains a registry of all its orbitals
- Data requests flow from orbital to collection to container
- The orbital applies transforms to the data from the collection

### Sale to Collection Relationship
- The sale alkane has permission to call the collection's mint API
- The collection alkane validates requests from the sale alkane
- The sale alkane enforces limits on the number of instances
- The collection alkane creates instances when requested by the sale alkane

### Orbital to Orbitals Support Relationship
- The orbital alkane implements the Orbital trait
- The orbital uses the BytesTransform trait for data transformations
- The orbital can use custom transform implementations
- The orbitals-support crate provides default implementations

### Orbital to Orbital Macros Relationship
- The orbital alkane uses the declare_orbital! macro for WebAssembly interface
- The orbital defines an OrbitalMessage enum with the OrbitalMessage derive macro
- The OrbitalMessage derive macro implements the MessageDispatch trait
- The orbital-macros crate handles the boilerplate code for WebAssembly integration
- The macros maintain compatibility with the alkanes framework

### Data Flow
1. User requests data from an orbital instance (opcode 1000)
2. The orbital proxies the request to its collection
3. The collection retrieves the base data from the container
4. The orbital applies a transform based on its index and sequence number
5. The transformed data is returned to the user

### Purchase Flow
1. User sends payment to the sale alkane (opcode 77)
2. The sale alkane verifies the payment amount and alkane ID
3. The sale alkane checks if the instance limit has been reached
4. The sale alkane calls the collection alkane's API to create a new orbital instance
5. The collection alkane creates the instance with the next available sequence number
6. The sale alkane returns the newly created orbital instance to the user
7. The sale alkane provides change for excess payment

## Technical Patterns

### WASM Optimization
- The container alkane is optimized for minimal size and maximum efficiency
- A WAT template is used for direct control over the WASM structure
- Static data is efficiently packed into the WASM binary
- The __execute function returns data in the correct format
- The container is generated using a TypeScript library

### Special Call Pattern
- The collection alkane uses a special call function for container initialization
- This function avoids using __returndatacopy to save fuel costs
- It only uses the __call host function directly
- This is particularly useful for large response bodies

### Opcode Standardization
- All components use consistent opcode numbering
- Standard operations (0, 99-101, 1000) are implemented across components
- The sale alkane implements opcode 77 for purchasing
- The collection alkane implements opcode 102 for orbital count
- The sale alkane implements opcode 104 for terms of service
- The MessageDispatch trait is implemented via the OrbitalMessage derive macro
- The declare_orbital! macro is used for orbital alkane runtime integration
- The declare_alkane! macro is used for other alkanes' runtime integration

### Deployment Pattern
- The container WASM is attached to the collection deployment
- The collection is deployed using the oyl alkane new-contract command
- The orbital template is deployed separately if custom transforms are needed
- The sale alkane is deployed using the oyl alkane execute command
- The oyl CLI tool is used for all deployment operations