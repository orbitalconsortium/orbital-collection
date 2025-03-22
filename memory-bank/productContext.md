# Product Context

## Purpose
The orbital-collection project exists to provide a flexible and efficient system for creating and managing collections of orbital alkanes. It implements a factory pattern where a collection alkane can spawn multiple orbital instances, each with a unique sequence number, and apply transforms to the data based on that sequence number. The system also includes a sale mechanism to handle payments and limit the number of instances that can be created, as well as a support library for implementing custom transforms.

## Problem Statement
Creating collections of related but unique tokens in the alkane ecosystem requires a standardized approach that is both efficient and flexible. Additionally, there needs to be a secure way to handle sales and limit the supply of these tokens. This project addresses these needs by implementing a collection system with integrated sales functionality and a framework for custom data transformations.

## User Experience Goals
- Users should be able to create collections of related but unique orbitals
- The system should efficiently handle data transforms based on sequence numbers
- All standard token functionality should be available and compatible with existing systems
- The container alkane should provide efficient static data storage
- Users should be able to purchase orbital instances by paying a specified price
- The system should enforce limits on the total supply of orbital instances
- Developers should be able to create custom transforms for their orbitals
- The deployment process should be straightforward using the oyl CLI tool

## Key Features
1. **Collection Factory System**
   - Collection alkane acts as a factory for orbital instances
   - Tracking of all instances created by the collection
   - Proper initialization sequence with authentication
   - Data opcode proxying with transforms
   - API for the sale alkane to mint new instances
   - Special call function for container initialization
   - Container sequence tracking

2. **Orbital Instance System**
   - Standard token functionality (name, symbol, total supply)
   - Compatibility with the collection's factory system
   - Sequence-based transforms for unique data
   - Consistent opcode format
   - Superscript index display in name and symbol
   - Implementation of the Orbital trait

3. **Container Optimization**
   - TypeScript library for generating minimal WASM containers
   - WAT template for efficient implementation
   - Browser and Node.js interfaces
   - Efficient data embedding mechanism
   - Proper CallResponse structure
   - wabt.js integration for browser compatibility

4. **Sale System**
   - Payment processing for orbital instance creation
   - Configurable price per instance
   - Supply limit enforcement
   - Integration with the collection's factory API
   - Support for different payment alkanes
   - Bulk purchasing with change calculation
   - Terms of service for legal protection

5. **Transform Framework**
   - BytesTransform trait for custom data transformations
   - Orbital trait with default implementations
   - Example implementations for custom transforms
   - Documentation for custom transform implementation
   - Support for image transformations and other data types

6. **Deployment System**
   - Detailed instructions for using the oyl CLI tool
   - Correct commands for deploying all components
   - Instructions for purchasing orbitals
   - Instructions for viewing orbital data
   - Support for custom orbital templates

## Stakeholders
- Developers creating collections of related but unique tokens
- Users purchasing and interacting with the orbital instances
- System integrators building on top of the alkane ecosystem
- Performance engineers concerned with WASM efficiency
- Collection creators who want to sell their orbitals
- Developers implementing custom transforms for their orbitals
- Deployment engineers using the oyl CLI tool

## Use Cases

### Creating a Collection
1. Developer generates a container WASM with their base data
2. Developer deploys the collection with the container WASM attached
3. Developer deploys a custom orbital template if needed
4. Developer deploys the sale alkane with the collection reference
5. Users can now purchase orbitals from the sale alkane

### Purchasing Orbitals
1. User sends payment to the sale alkane with the Purchase opcode
2. Sale alkane verifies the payment and calculates how many orbitals can be purchased
3. Sale alkane calls the collection to create the orbital instances
4. Sale alkane returns the orbitals and any change to the user

### Viewing Orbital Data
1. User requests data from an orbital with the GetData opcode
2. Orbital proxies the request to its collection
3. Collection retrieves the base data from the container
4. Orbital applies a transform based on its index and sequence number
5. Transformed data is returned to the user

### Creating Custom Transforms
1. Developer implements the BytesTransform trait
2. Developer creates a custom orbital that uses the transform
3. Developer deploys the custom orbital template
4. Developer deploys the collection with the custom orbital template
5. Users can now purchase orbitals with the custom transform