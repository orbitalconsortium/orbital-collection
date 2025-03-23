# Orbitals Container Generator

A TypeScript library and CLI tool for generating WebAssembly (WASM) container files for orbital collections. This tool embeds data into a WebAssembly module using a template, making it easy to create data containers for the orbital-collection system.

## Features

- Generate WASM containers from any file type
- Support for both browser and Node.js environments
- Customizable WebAssembly template
- Command-line interface for easy integration into workflows
- Proper handling of both relative and absolute file paths
- Automatic creation of output directories

## Installation

### Global Installation

```bash
npm install -g orbitals-container-generator
```

### Local Installation

```bash
npm install orbitals-container-generator
```

## CLI Usage

The CLI tool can be used to generate WASM containers from any file:

```bash
# Using global installation
orbitals-container-generate generate <input-file> -o <output-file>

# Using local installation
npx orbitals-container-generate generate <input-file> -o <output-file>

# Using from the project directory
node ./dist/src/cli.js generate <input-file> -o <output-file>
```

### Options

- `-o, --output <output>`: Output file path (default: "container.wasm")
- `-t, --template <template>`: Custom template WAT file path

### Examples

```bash
# Generate a container from an image file
orbitals-container-generate generate image.png -o container.wasm

# Generate a container with a custom template
orbitals-container-generate generate data.json -o container.wasm -t custom-template.wat

# Using relative paths
orbitals-container-generate generate ./data/image.png -o ./output/container.wasm

# Output to a subdirectory (will be created if it doesn't exist)
orbitals-container-generate generate image.png -o output/subdir/container.wasm
```

## API Usage

### Browser Usage

```javascript
import { generateContainerFromFile, wabtWat2Wasm } from 'orbitals-container-generator';

// Get a file from an input element
const fileInput = document.getElementById('fileInput');
const file = fileInput.files[0];

// Generate a container
const wasm = await generateContainerFromFile(file, wabtWat2Wasm);

// Use the generated WASM
// ...
```

### Node.js Usage

```javascript
const fs = require('fs');
const path = require('path');
const wabt = require('wabt');
const { generateContainerFromData } = require('orbitals-container-generator');

async function generateContainer() {
  // Read the input file
  const inputData = fs.readFileSync('input.png');
  
  // Initialize wabt
  const wabtInstance = await wabt.init();
  
  // Define the wat2wasm function
  const wat2wasm = async (wat) => {
    const module = wabtInstance.parseWat('container.wat', wat);
    const { buffer } = module.toBinary({});
    module.destroy();
    return new Uint8Array(buffer);
  };
  
  // Generate the container
  const wasm = await generateContainerFromData(
    new Uint8Array(inputData),
    wat2wasm
  );
  
  // Write the output file
  fs.writeFileSync('container.wasm', Buffer.from(wasm));
}

generateContainer();
```

## Container Format

The generated WASM container has the following structure:

1. A memory section with the embedded data
2. An `__execute` function that returns a pointer to a CallResponse structure
3. The CallResponse structure has the format:
   - 16 bytes for alkanes count (always 0 for containers)
   - The embedded data

## Development

### Prerequisites

- Node.js 14 or higher
- npm or yarn

### Setup

```bash
# Clone the repository
git clone https://github.com/your-org/orbitals-container-generator.git
cd orbitals-container-generator

# Install dependencies
npm install

# Build the project
npm run build
```

### Testing

```bash
# Run tests
npm test

# Run tests with coverage
npm run test:coverage

# Run tests in watch mode
npm run test:watch
```

## License

MIT