#!/usr/bin/env node
"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
const commander_1 = require("commander");
const fs = __importStar(require("fs"));
const path = __importStar(require("path"));
const index_1 = require("./index");
// Define the program
const program = new commander_1.Command();
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
    .action(async (input, options) => {
    try {
        // Resolve the input path (handles both relative and absolute paths)
        const resolvedInputPath = path.resolve(input);
        // Check if the input file exists
        if (!fs.existsSync(resolvedInputPath)) {
            console.error(`Error: Input file '${input}' does not exist`);
            process.exit(1);
        }
        // Import wabt dynamically
        const wabtModule = await Promise.resolve().then(() => __importStar(require('wabt')));
        const wabtInstance = await wabtModule.default();
        // Define the wat2wasm function
        const wat2wasm = async (wat) => {
            const module = wabtInstance.parseWat('container.wat', wat);
            const { buffer } = module.toBinary({});
            module.destroy();
            return new Uint8Array(buffer);
        };
        // Resolve the template path if provided
        const templatePath = options.template ? path.resolve(options.template) : undefined;
        // Generate the container
        const containerOptions = templatePath ? { template: templatePath } : {};
        const wasm = await (0, index_1.generateContainerFromFilePath)(resolvedInputPath, wat2wasm, containerOptions);
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
    }
    catch (error) {
        console.error(`Error: ${error instanceof Error ? error.message : String(error)}`);
        process.exit(1);
    }
});
// Parse the command line arguments
program.parse(process.argv);
