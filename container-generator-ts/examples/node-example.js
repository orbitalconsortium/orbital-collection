#!/usr/bin/env node

/**
 * Example of using the orbitals-container-generator in Node.js
 * 
 * Usage:
 *   node node-example.js <input-file> <output-file>
 * 
 * Example:
 *   node node-example.js image.png container.wasm
 */

const fs = require('fs');
const path = require('path');
const wabt = require('wabt');
const { generateContainerFromData } = require('../dist/index');

// Parse command line arguments
const inputFile = process.argv[2];
const outputFile = process.argv[3] || 'container.wasm';

if (!inputFile) {
  console.error('Error: Input file is required');
  console.error('Usage: node node-example.js <input-file> <output-file>');
  process.exit(1);
}

// Check if the input file exists
if (!fs.existsSync(inputFile)) {
  console.error(`Error: Input file '${inputFile}' does not exist`);
  process.exit(1);
}

// Main function
async function main() {
  try {
    // Read the input file
    const inputData = fs.readFileSync(inputFile);
    
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
      wat2wasm,
      { template: path.join(__dirname, '..', 'template.wat') }
    );
    
    // Write the output file
    fs.writeFileSync(outputFile, Buffer.from(wasm));
    
    console.log(`Container WASM file generated successfully: ${outputFile} (${wasm.length} bytes)`);
  } catch (error) {
    console.error(`Error: ${error.message}`);
    process.exit(1);
  }
}

// Run the main function
main();