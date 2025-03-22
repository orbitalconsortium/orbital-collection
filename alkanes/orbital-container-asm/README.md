# Orbital Container Assembler

A TypeScript library for generating WebAssembly (WASM) containers that store static data for the orbital-collection system. The container responds to the Data opcode (1000) and returns the embedded data.

## Overview

The orbital-container-asm component provides a TypeScript library for generating container WASM files. It's designed to be used in both browser and Node.js environments, with a focus on browser compatibility.

## Installation

```bash
# Install from npm
npm install orbital-container-asm

# Or if you're working with the local repository
npm install
npm run build
```

## Usage

### Browser

```html
<!-- Load the wabt.js library -->
<script src="https://cdn.jsdelivr.net/npm/wabt@1.0.32/index.js"></script>

<!-- Load the orbital-container-asm library -->
<script type="module">
  import { 
    generateContainerFromFile,
    wabtWat2Wasm
  } from 'orbital-container-asm';

  // When the user selects a file
  fileInput.addEventListener('change', async (e) => {
    const file = e.target.files[0];
    if (!file) return;
    
    try {
      // Generate the WASM using the library
      const wasmBinary = await generateContainerFromFile(file, wabtWat2Wasm);
      
      // Create a Blob from the WASM binary
      const wasmBlob = new Blob([wasmBinary], { type: 'application/wasm' });
      
      // Download the WASM file
      const url = URL.createObjectURL(wasmBlob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'container.wasm';
      a.click();
      URL.revokeObjectURL(url);
    } catch (error) {
      console.error('Error:', error);
    }
  });
</script>
```

### Node.js

```typescript
import * as fs from 'fs';
import { generateWat } from 'orbital-container-asm';

// Read the input file
const data = fs.readFileSync('input.png');

// Generate the WAT file
const wat = generateWat(new Uint8Array(data));

// Write the WAT file
fs.writeFileSync('output.wat', wat);

// To convert the WAT to WASM, you would need to use a library like wabt
// or use the wat2wasm command-line tool:
// wat2wasm output.wat -o output.wasm
```

## API

### `generateWat(data: Uint8Array, options?: ContainerOptions): string`

Generates a WebAssembly Text Format (WAT) file with the provided data embedded.

- `data`: The data to embed in the WAT file
- `options`: Optional configuration options
  - `template`: Custom WAT template to use instead of the default

Returns the WAT file content as a string.

### `generateWasm(data: Uint8Array, wat2wasm: Wat2Wasm, options?: ContainerOptions): Promise<Uint8Array>`

Generates a WebAssembly (WASM) file with the provided data embedded.

- `data`: The data to embed in the WASM file
- `wat2wasm`: Function to convert WAT to WASM
- `options`: Optional configuration options
  - `template`: Custom WAT template to use instead of the default

Returns a Promise that resolves to the WASM file content as a Uint8Array.

### `generateContainerFromFile(file: File, wat2wasm?: Wat2Wasm, options?: ContainerOptions): Promise<Uint8Array>`

Generates a container WASM file from a File object.

- `file`: File object to embed in the WASM
- `wat2wasm`: Function to convert WAT to WASM (defaults to placeholder)
- `options`: Optional configuration options
  - `template`: Custom WAT template to use instead of the default

Returns a Promise that resolves to the WASM file content as a Uint8Array.

### `generateContainerFromData(data: Uint8Array, wat2wasm?: Wat2Wasm, options?: ContainerOptions): Promise<Uint8Array>`

Generates a container WASM file from raw data.

- `data`: The data to embed in the WASM
- `wat2wasm`: Function to convert WAT to WASM (defaults to placeholder)
- `options`: Optional configuration options
  - `template`: Custom WAT template to use instead of the default

Returns a Promise that resolves to the WASM file content as a Uint8Array.

### `wabtWat2Wasm(wat: string): Promise<Uint8Array>`

Converts WAT to WASM using the wabt.js library.

- `wat`: WAT code to convert

Returns a Promise that resolves to the WASM binary.

## Examples

The `examples` directory contains:

- `index.html`: A web page that demonstrates how to use the library in a browser
- `node-example.ts`: A Node.js script that demonstrates how to use the library in Node.js

To run the browser example:

```bash
# Build the library
npm run build

# Serve the examples directory
npx serve .

# Open http://localhost:5000/examples/ in your browser
```

To run the Node.js example:

```bash
# Install dependencies
npm install
npm install -g ts-node

# Run the example
ts-node examples/node-example.ts input.png output.wat
```

## How It Works

The container is generated from a WebAssembly Text Format (WAT) template. The template defines a minimal WASM module that:

1. Exports a memory section
2. Embeds the data in the memory
3. Exports a `__execute` function that returns a pointer to a serialized `CallResponse` structure

When the `__execute` function is called, it:

1. Creates a `CallResponse` with empty alkanes and the embedded data
2. Serializes the `CallResponse` according to the alkane protocol
3. Returns a pointer to the serialized data

## Integration with Orbital Collection

In the orbital-collection system:

1. The container alkane is deployed first
2. The collection alkane is deployed with a reference to the container
3. When an orbital instance receives a Data opcode (1000), it proxies the request to its collection
4. The collection retrieves the base data from the container
5. The collection applies a transform based on the orbital's sequence number
6. The transformed data is returned to the user

This component focuses on creating the most efficient container possible, optimized for minimal size and maximum performance.