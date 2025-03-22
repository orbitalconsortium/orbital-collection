# orbital-collection

## Overview
A monorepo for orbital-collection, implementing a set of alkanes (WASM programs) that work together to create a collection system. The collection alkane can factory up instances of the orbital alkane and keep track of them, with the Data opcode (1000) proxying back to the collection alkane but applying transforms to render the final bytearray. The system also includes a TypeScript library for generating container WASM files and a support library for implementing custom transforms.

## Core Requirements

### Collection Alkane
- ✅ Must implement factory functionality to spawn orbital instances
- ✅ Must track all instances it creates
- ✅ Must support proper initialization sequence with authentication
- ✅ Must implement the Data opcode (1000) to proxy back to the collection with transforms
- ✅ Must provide an API for the sale alkane to mint new orbital instances
- ✅ Must use a special call function for container initialization to avoid fuel costs

### Orbital Alkane
- ✅ Must implement standard token functionality (name, symbol, total supply)
- ✅ Must be compatible with the collection alkane's factory system
- ✅ Must follow the owned token opcode format for consistency
- ✅ Must implement the Data opcode (1000) to apply transforms based on sequence number
- ✅ Must display name and symbol with superscript index
- ✅ Must implement the Orbital trait from orbitals-support

### Container Alkane
- ✅ Must implement a minimal WASM program that responds to the Data opcode (1000)
- ✅ Must efficiently return static data built into the WASM
- ✅ Must produce the correct CallResponse structure
- ✅ Must be built in the most efficient way possible, using a .wat template
- ✅ Must provide browser and Node.js interfaces for container generation
- ✅ Must use wabt.js for WAT to WASM conversion in the browser

### Sale Alkane
- ✅ Must accept a payment alkane ID (typically [2, 0]) during initialization
- ✅ Must set a price for each orbital instance
- ✅ Must enforce a limit on the total number of orbital instances that can be created
- ✅ Must have permission to mint via the collection alkane API
- ✅ Must handle payment processing and orbital creation in a single transaction
- ✅ Must support bulk purchasing with change calculation
- ✅ Must include terms of service for legal protection

### Orbitals Support
- ✅ Must provide the BytesTransform trait for data transformations
- ✅ Must provide the Orbital trait with default implementations
- ✅ Must include example implementations for custom transforms
- ✅ Must provide documentation for custom transform implementation

## Opcode Specification 

### Standard Operations
- 0: Initialize() - Initialize the alkane
- 99: GetName() -> String
- 100: GetSymbol() -> String  
- 101: GetTotalSupply() -> u128
- 102: GetOrbitalCount() -> u128 (collection alkane only)
- 104: GetTermsOfService() -> String (sale alkane only)
- 1000: GetData() -> Vec<u8>

### Sale Alkane Operations
- 77: Purchase() - Purchase an orbital instance by paying the specified price

## Technical Implementation
- ✅ Use MessageDispatch derive macro for opcode handling
- ✅ Use declare_alkane! macro for proper runtime integration
- ✅ Implement sequence-based transforms for orbital instances
- ✅ Use the orbital-container-asm program to pack static data efficiently
- ✅ Implement payment processing in the sale alkane
- ✅ Provide detailed deployment instructions using the oyl CLI tool

## Architecture Pattern
The system follows a factory pattern where:
1. The collection alkane acts as a factory for orbital instances
2. Each orbital instance has a unique sequence number (address [2, n])
3. The Data opcode proxies back to the collection but applies transforms based on sequence number
4. The container alkane provides efficient static data storage
5. The sale alkane handles payment processing and interacts with the collection alkane to create new orbital instances
6. The orbitals-support library provides traits and utilities for implementing custom transforms

## Security Patterns
- ✅ Call observe_initialization() in Initialize operation
- ✅ Ensure proper authentication for factory operations
- ✅ Implement proper error handling for all operations
- ✅ Validate payment amounts in the sale alkane
- ✅ Enforce instance limits in both collection and sale alkanes
- ✅ Include terms of service for legal protection

## Deployment Pattern
- ✅ The container WASM is attached to the collection deployment
- ✅ The collection is deployed using the oyl alkane new-contract command
- ✅ The orbital template is deployed separately if custom transforms are needed
- ✅ The sale alkane is deployed using the oyl alkane execute command
- ✅ The oyl CLI tool is used for all deployment operations

## Transform Pattern
- ✅ The BytesTransform trait provides a standard interface for data transformations
- ✅ The Orbital trait provides default implementations for common functionality
- ✅ Custom transforms can be implemented by implementing the BytesTransform trait
- ✅ The transform is applied after retrieving data from the collection
- ✅ The transform is based on the orbital's index and sequence number