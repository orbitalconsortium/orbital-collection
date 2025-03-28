# Active Context

## Current Focus

We are currently focused on deploying and testing the orbital collection system with Bitcoin integration. The main components are:

1. **Bitcoin Collection Alkane**: A collection alkane that integrates with Bitcoin for payments
2. **Bitcoin Sale Alkane**: A sale alkane that processes Bitcoin payments and creates orbital instances
3. **Container Generator**: A TypeScript tool for generating container WASM files from images
4. **Deployment Script**: A bash script for automating the deployment process

## Recent Changes

- Implemented the bitcoin-collection alkane for Bitcoin-based collections
- Implemented the bitcoin-sale alkane for Bitcoin-based sales
- Created a deployment script (deploy-orbital-collection.sh) for automating the deployment process
- Created a BTC payment helper script (btc-payment-helper.js) for purchasing orbitals with BTC
- Updated documentation in the memory bank

## Next Steps

1. **Testing**: Conduct thorough testing of the deployment process and the Bitcoin integration
2. **Optimization**: Optimize the container WASM size and transform logic performance
3. **Documentation**: Complete the API documentation and user guide
4. **UI**: Develop a user interface for interacting with the orbital collection system

## Active Decisions and Considerations

### Deployment Process

We've decided to create a comprehensive deployment script that automates the entire process, from initializing the regtest environment to purchasing an orbital with BTC. This script handles:

- Initializing the regtest environment
- Funding the necessary addresses
- Deploying all contracts
- Setting up the Bitcoin sale instance
- Purchasing an orbital with BTC

The script is designed to be flexible, allowing users to provide their own mnemonic and image path.

### Bitcoin Integration

We've integrated Bitcoin payments into the sale alkane, allowing users to purchase orbitals with BTC. This integration includes:

- Processing Bitcoin payments to a taproot address
- Verifying payment amounts
- Creating orbital instances in response to payments

### Container Generation

We're using the container-generator-ts tool to generate container WASM files from images. This tool:

- Takes an image as input
- Embeds the image data in a WAT template
- Converts the WAT to WASM
- Outputs a container WASM file that can be deployed

### Known Issues

We're aware of an "Insufficient Balance" error that can occur when deploying contracts with the oyl alkane new-contract command. This is a known issue with the oyl-sdk, and we've documented workarounds in the deploymentInstructions.md file.

## Current Status

The project is in a functional state, with all core components implemented and a deployment script created. We're now focusing on testing, optimization, and documentation.

## Next Meeting Agenda

1. Review the deployment process and address any issues
2. Discuss optimization strategies for the container WASM size
3. Plan the development of a user interface
4. Assign tasks for completing the documentation