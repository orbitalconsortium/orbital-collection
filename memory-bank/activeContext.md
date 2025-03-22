# Active Context

## Current Focus
The current focus is on optimizing and enhancing the orbital-collection system with its four main components: the collection alkane (factory), the orbital alkane (instances), the container alkane (data storage), and the sale alkane (payment processing). We've implemented all components with a particular focus on the container alkane's efficiency and flexibility, reimplementing it as a TypeScript library. We've also created a new orbitals-support crate that provides traits and utilities for implementing orbital alkanes.

## Recent Changes
- **Complete Implementation**: Implemented all four main components
  - Collection alkane in alkanes/collection/
  - Orbital alkane in alkanes/collection-child/
  - Container alkane in alkanes/orbital-container-asm/ (TypeScript library)
  - Sale alkane in alkanes/sale/
- **Container Alkane Reimplementation**: Converted to TypeScript library
  - Created WAT template with proper memory layout
  - Implemented browser interface for container generation
  - Added Node.js example for command-line usage
  - Ensured proper CallResponse serialization
- **Orbitals Support Crate**: Created a new crate for orbital alkane support
  - Implemented BytesTransform trait for data transformations
  - Implemented Orbital trait with default implementations
  - Added example implementations for custom transforms
- **Special Call Function**: Implemented a special call function for container initialization
  - Avoids fuel costs for large response bodies
  - Uses __call host function directly without __returndatacopy
- **Deployment Instructions**: Added detailed deployment instructions using the oyl CLI tool
- **Project Structure**: Set up the monorepo structure with workspace configuration
- **Documentation**: Updated Memory Bank files and created README.md

## Active Decisions

### Container Alkane Implementation
- Reimplemented as a TypeScript library for better browser compatibility
- Created a WAT template that correctly implements the memory layout
- Used the wabt.js library for WAT to WASM conversion in the browser
- Provided multiple interfaces (browser, TypeScript API, Node.js)
- Focused on direct WAT compilation for minimal size and maximum efficiency

### Transform Logic
- Implemented BytesTransform trait for customizable transforms
- Created IdentityTransform for default pass-through behavior
- Added example implementations for custom transforms
- Provided a framework for developers to create their own transforms

### Orbital Implementation
- Implemented Orbital trait with default implementations
- Made name and symbol display the collection's name/symbol with superscript index
- Added proxy functionality for Data opcode with transform application
- Stored collection reference and index for proper operation

### Sale Alkane Implementation
- Implemented payment verification and change calculation
- Added bulk purchasing capability
- Included terms of service for legal protection
- Used proper parameter types (u128 values for AlkaneId components)

### Deployment Process
- Documented the correct deployment process using the oyl CLI tool
- Explained how to deploy the collection with the container WASM attached
- Described how to deploy a custom orbital template
- Detailed how to deploy the sale alkane and purchase orbitals

## Next Steps

### Implementation Tasks
1. **Transform Logic**
   - Implement more example transforms for common use cases
   - Consider using a PNG library for image transformations
   - Create documentation for custom transform implementation

2. **Container Optimization**
   - Test the TypeScript container generation with different data types and sizes
   - Benchmark performance of the generated containers
   - Consider further optimizations for the WAT template

3. **Security Enhancements**
   - Implement proper authorization checks in the collection alkane
   - Enhance payment verification in the sale alkane
   - Add more comprehensive error handling

4. **Testing Improvements**
   - Add more comprehensive unit tests
   - Implement integration tests with actual components
   - Create test utilities for easier testing

5. **Deployment Scripts**
   - Add scripts for automating the deployment process
   - Create more detailed documentation for deployment
   - Consider containerization for easier deployment

### Open Questions
- What additional example transforms should we provide?
- Should we use a specific PNG library for image transformations?
- What additional optimizations can be made to the container alkane?
- How should we handle authorization between components?
- What additional security measures should we implement?
- How can we make the testing process more efficient?