#!/usr/bin/env node

import { Command } from 'commander';
import * as fs from 'fs';
import * as path from 'path';
import { generateContainerFromFilePath, Wat2Wasm, dirPath } from './index';

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
      // Resolve the input path (handles both relative and absolute paths)
      const resolvedInputPath = path.resolve(input);
      
      // Check if the input file exists
      if (!fs.existsSync(resolvedInputPath)) {
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

      // Resolve the template path if provided
      const templatePath = options.template ? path.resolve(options.template) : undefined;
      
      // Generate the container
      const containerOptions = templatePath ? { template: templatePath } : {};
      const wasm = await generateContainerFromFilePath(resolvedInputPath, wat2wasm, containerOptions);

      // Resolve the output path (handles both relative and absolute paths)
      const resolvedOutputPath = path.resolve(options.output);
      
      // Create the directory if it doesn't exist
      const outputDir = path.dirname(resolvedOutputPath);
      if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir, { recursive: true });
      }
      
      // Write the output file
      fs.writeFileSync(resolvedOutputPath, Buffer.from(wasm));

      console.log(`Container WASM file generated successfully: ${resolvedOutputPath}`);
    } catch (error: unknown) {
      console.error(`Error: ${error instanceof Error ? error.message : String(error)}`);
      process.exit(1);
    }
  });

// Parse the command line arguments
program.parse(process.argv);