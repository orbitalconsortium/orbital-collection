#!/usr/bin/env node

import { Command } from 'commander';
import * as fs from 'fs';
import * as path from 'path';
import { generateContainerFromFilePath, Wat2Wasm, currentDir } from './index';

// Define the program
const program = new Command();
program
  .name('orbitals-container-generate')
  .description('Generate a container WASM file for orbital collections')
  .version('0.1.0');

// Add the generate command
program
  .command('generate')
  .description('Generate a container WASM file from a data file')
  .argument('<input>', 'Input file path')
  .option('-o, --output <output>', 'Output file path', 'container.wasm')
  .option('-t, --template <template>', 'Template WAT file path')
  .action(async (input: string, options: { output: string; template?: string }) => {
    try {
      // Check if the input file exists
      if (!fs.existsSync(input)) {
        console.error(`Error: Input file '${input}' does not exist`);
        process.exit(1);
      }

      // Import wabt dynamically
      const wabtModule = await import('wabt');
      const wabtInstance = await wabtModule.default();

      // Define the wat2wasm function
      const wat2wasm: Wat2Wasm = async (wat: string): Promise<Uint8Array> => {
        const module = wabtInstance.parseWat('container.wat', wat);
        const { buffer } = module.toBinary({});
        module.destroy();
        return new Uint8Array(buffer);
      };

      // Generate the container
      const containerOptions = options.template ? { template: options.template } : {};
      const wasm = await generateContainerFromFilePath(input, wat2wasm, containerOptions);

      // Write the output file
      fs.writeFileSync(options.output, Buffer.from(wasm));

      console.log(`Container WASM file generated successfully: ${options.output}`);
    } catch (error: unknown) {
      console.error(`Error: ${error instanceof Error ? error.message : String(error)}`);
      process.exit(1);
    }
  });

// Parse the command line arguments
program.parse(process.argv);