# WIP: Don't use in production (yet)

# Orbital Collection

A comprehensive framework for creating and managing NFT collections on the Alkanes platform. This project provides a complete solution for generating, deploying, and selling non-fungible digital assets with customizable rendering.

## Architecture

The Orbital Collection framework consists of four main components:

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│                 │     │                 │     │                 │
│  Sale Alkane    │────▶│  Collection     │────▶│  Container      │
│                 │     │  Alkane         │     │  Alkane         │
│                 │     │                 │     │                 │
└────────┬────────┘     └────────┬────────┘     └─────────────────┘
         │                       │
         │                       │
         │                       ▼
         │              ┌─────────────────┐
         └─────────────▶│  Orbital        │
                        │  Alkane         │
                        │                 │
                        └─────────────────┘
```

### Data Flow

1. **Container** stores the base data for the collection
2. **Collection** acts as a factory for orbital instances and proxies data requests to the container
3. **Orbital** instances proxy data requests to the collection and apply transforms
4. **Sale** handles payment processing and orbital creation

## Components

### Container Alkane (TypeScript)

The container alkane is a TypeScript library that generates minimal WebAssembly containers for storing the base data of the collection. It provides:

- Browser-compatible container generation
- Efficient static data storage
- Multiple interfaces (browser, TypeScript API, Node.js)

### Collection Alkane (Rust)

The collection alkane acts as a factory for orbital instances and proxies data requests to the container. It provides:

- Orbital instance creation
- Data proxying to the container
- Registry of created instances

### Orbital Alkane (Rust)

The orbital alkane represents an instance of the collection and proxies data requests to the collection. It provides:

- Data proxying to the collection
- Custom data transformations
- Unique identification with superscript indices

### Sale Alkane (Rust)

The sale alkane handles payment processing and orbital creation. It provides:

- Fixed price minting
- Payment verification
- Bulk purchasing with change calculation
- Terms of service

### Orbitals Support (Rust)

The orbitals-support crate provides traits and utilities for implementing orbital alkanes. It provides:

- BytesTransform trait for custom data transformations
- Orbital trait with default implementations
- Example implementations for developers

## End-to-End Deployment Guide

This guide walks through the complete process of deploying an orbital collection sale using the oyl CLI tool.

### Prerequisites

- Install the oyl CLI tool
- Compile the collection, orbital, and sale alkanes
- Prepare your base image/data for the container

### Step 1: Generate the Container WASM

First, generate the container WASM file that will store your base data:

```bash
# Using the browser interface
open alkanes/orbital-container-asm/examples/index.html
# Upload your data file and download the generated container WASM

# OR using the Node.js interface
cd alkanes/orbital-container-asm
npm install
node examples/node-example.js /path/to/your/data.file /path/to/output/container.wasm
```

### Step 2: Deploy the Collection with Container

Deploy the collection alkane with the container WASM attached:

```bash
cargo build --release --package orbitals-collection-contract
oyl alkane new-contract -c ./container.wasm -data 6,COLLECTION_TEMPLATE_NUMBER,0,name_part1,name_part2,symbol
```

Where:
- `COLLECTION_TEMPLATE_NUMBER` is the template ID for the collection
- `name_part1` and `name_part2` are u128 values representing your collection name
- `symbol` is a u128 value representing your collection symbol

This will deploy the collection alkane with the container WASM attached and return a transaction ID and vout. Note these values as they will be used to reference the collection.

### Step 3: Deploy the Orbital Template

If you want to use a custom orbital template with a different transform than the default one, deploy it using:

```bash
cargo build --release --package orbitals-orbital-contract
oyl alkane new-contract -c ./target/release/orbitals_orbital_contract.wasm -data 3,ORBITAL_TEMPLATE_ID,100
```

Where:
- `ORBITAL_TEMPLATE_ID` is your chosen template ID for the orbital
- `100` is an opcode that does nothing (or any other opcode that does nothing)

This will deploy the orbital template and return a transaction ID and vout. Note the values as they will be used for the `ORBITAL_TEMPLATE_ID` constant.

### Step 4: Deploy the Sale Alkane

Deploy the sale alkane using:

```bash
cargo build --release --package orbitals-sale-contract
oyl alkane execute -data 6,SALE_TEMPLATE_ID,0,collection_block,collection_tx,payment_block,payment_tx,price,limit
```

Where:
- `SALE_TEMPLATE_ID` is the template ID for the sale
- `collection_block` and `collection_tx` are the block and tx values of the collection alkane
- `payment_block` and `payment_tx` are the block and tx values of the payment alkane
- `price` is the price per orbital in the payment alkane's units
- `limit` is the maximum number of orbitals that can be sold (0 for unlimited)

This will deploy the sale alkane and return a transaction ID and vout. Note these values as they will be used to reference the sale.

### Step 5: Purchase Orbitals

To purchase orbitals, send payment to the sale alkane with the Purchase opcode:

```bash
oyl alkane execute -data 2,sale_tx,77 -e payment_block:payment_tx:amount:1
```

Where:
- `sale_tx` is the tx value of the sale alkane
- `payment_block` and `payment_tx` are the block and tx values of the payment alkane
- `amount` is the amount of payment to send

The sale alkane will:
1. Verify the payment
2. Calculate how many orbitals can be purchased
3. Create the orbitals through the collection
4. Return the orbitals and any change

### Step 6: View Orbital Data

To view the data of an orbital, use the `simulate` command to call the Data opcode as a view function:

```bash
oyl alkane simulate -target "2:orbital_tx" -inputs "1000" -decoder "default"
```

Where:
- `orbital_tx` is the tx value of the orbital alkane

The orbital will:
1. Proxy the request to its collection
2. Apply a transform based on its index
3. Return the transformed data

## Custom Transforms

To create a custom transform for your orbitals, implement the BytesTransform trait:

```rust
use orbitals_support::BytesTransform;

pub struct CustomTransform;

impl BytesTransform for CustomTransform {
    fn transform(&self, input: &[u8], index: u128, sequence: u128) -> Vec<u8> {
        // Apply your custom transformation here
        // For example, if working with images:
        // 1. Parse the input bytes as an image
        // 2. Apply transformations based on the index and sequence
        // 3. Encode the transformed image back to bytes
        
        // For now, just return the input bytes unchanged
        input.to_vec()
    }
}
```

Then create a custom orbital that uses your transform:

```rust
use orbitals_support::{Orbital, BytesTransform};

#[derive(Default)]
pub struct CustomOrbital(());

impl Orbital for CustomOrbital {
    fn get_transform(&self) -> Box<dyn BytesTransform> {
        // Use your custom transform
        Box::new(CustomTransform)
    }
}
```

## Development

### Prerequisites

- Rust 2021 edition
- Node.js and npm
- WebAssembly Binary Toolkit (WABT)
- oyl CLI tool

### Building

1. Build the TypeScript library:

```bash
cd alkanes/orbital-container-asm
npm install
npm run build
```

2. Build the Rust components:

```bash
cargo build --release --package orbitals-collection-contract
cargo build --release --package orbitals-orbital-contract
cargo build --release --package orbitals-sale-contract
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.