import * as fs from 'fs';
import * as path from 'path';
import { generateWat } from '../src';

// This is a simple example of how to use the orbital-container-asm library in Node.js
// Note that this example only generates the WAT file, not the WASM file,
// since the library is designed to be browser-compatible and doesn't include
// a Node.js-specific wat2wasm implementation.

// In a real Node.js application, you would need to use a library like wabt
// to convert the WAT to WASM, or use the wat2wasm command-line tool.

// Parse command line arguments
const args = process.argv.slice(2);

if (args.length < 2) {
  console.error('Usage: ts-node node-example.ts <input_file> <output_wat>');
  process.exit(1);
}

const inputPath = args[0];
const outputPath = args[1];

try {
  // Read the input file
  const data = fs.readFileSync(inputPath);
  
  // Generate the WAT file
  const wat = generateWat(new Uint8Array(data));
  
  // Write the WAT file
  fs.writeFileSync(outputPath, wat);
  
  console.log(`WAT file generated successfully: ${outputPath}`);
  
  // To convert the WAT to WASM, you would need to use a library like wabt
  // or use the wat2wasm command-line tool:
  console.log('\nTo convert the WAT to WASM, run:');
  console.log(`wat2wasm ${outputPath} -o ${path.parse(outputPath).name}.wasm`);
} catch (error) {
  console.error(`Error: ${error instanceof Error ? error.message : String(error)}`);
  process.exit(1);
}