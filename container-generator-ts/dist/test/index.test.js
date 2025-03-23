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
const fs = __importStar(require("fs"));
const path = __importStar(require("path"));
const os = __importStar(require("os"));
const index_1 = require("../src/index");
describe('Container Generator Library Tests', () => {
    let tempDir;
    beforeEach(() => {
        // Create a temporary directory for each test
        tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'orbitals-container-test-'));
    });
    afterEach(() => {
        // Clean up the temporary directory
        fs.rmSync(tempDir, { recursive: true, force: true });
    });
    describe('generateWat', () => {
        test('should generate a WAT file with embedded data', () => {
            // Create test data
            const testData = new Uint8Array([1, 2, 3, 4, 5]);
            // Generate WAT
            const wat = (0, index_1.generateWat)(testData);
            // Check that the WAT contains the data
            expect(wat).toContain('\\01\\02\\03\\04\\05');
            // Check that the WAT contains the data size (replaced with actual size)
            expect(wat).toContain('(i32.const 5)');
            // Check that the WAT is valid WebAssembly text format
            expect(wat).toContain('(module');
            expect(wat).toContain('(memory');
            expect(wat).toContain('(func');
            expect(wat).toContain('(export "__execute"');
        });
        test('should use a custom template if provided', () => {
            // Create test data
            const testData = new Uint8Array([1, 2, 3, 4, 5]);
            // Create a custom template
            const customTemplate = '(module (memory (export "memory") 1) (data (i32.const 0) "DATA_PLACEHOLDER"))';
            // Generate WAT with custom template
            const wat = (0, index_1.generateWat)(testData, { templateContent: customTemplate });
            // Check that the WAT uses the custom template
            expect(wat).toBe('(module (memory (export "memory") 1) (data (i32.const 0) "\\01\\02\\03\\04\\05"))');
        });
    });
    describe('generateWasm', () => {
        test('should generate a WASM file with embedded data', async () => {
            // Create test data
            const testData = new Uint8Array([1, 2, 3, 4, 5]);
            // Create a mock wat2wasm function
            const mockWat2Wasm = jest.fn().mockResolvedValue(new Uint8Array([0, 97, 115, 109])); // WASM magic number
            // Generate WASM
            const wasm = await (0, index_1.generateWasm)(testData, mockWat2Wasm);
            // Check that the mock was called
            expect(mockWat2Wasm).toHaveBeenCalled();
            // Check that the WASM is returned
            expect(wasm).toEqual(new Uint8Array([0, 97, 115, 109]));
        });
    });
    describe('generateContainerFromData', () => {
        test('should generate a container from data', async () => {
            // Create test data
            const testData = new Uint8Array([1, 2, 3, 4, 5]);
            // Create a mock wat2wasm function
            const mockWat2Wasm = jest.fn().mockResolvedValue(new Uint8Array([0, 97, 115, 109])); // WASM magic number
            // Generate container
            const wasm = await (0, index_1.generateContainerFromData)(testData, mockWat2Wasm);
            // Check that the mock was called
            expect(mockWat2Wasm).toHaveBeenCalled();
            // Check that the WASM is returned
            expect(wasm).toEqual(new Uint8Array([0, 97, 115, 109]));
        });
    });
    describe('defaultWat2Wasm', () => {
        test('should return a placeholder WASM binary', async () => {
            // Create test WAT
            const testWat = '(module)';
            // Mock console.warn to prevent output during tests
            const originalWarn = console.warn;
            console.warn = jest.fn();
            try {
                // Call defaultWat2Wasm
                const wasm = await (0, index_1.defaultWat2Wasm)(testWat);
                // Check that a warning was logged
                expect(console.warn).toHaveBeenCalled();
                // Check that a placeholder was returned
                expect(wasm.length).toBeGreaterThan(0);
                // Check that the placeholder contains the expected text
                const decoder = new TextDecoder();
                expect(decoder.decode(wasm)).toBe('Placeholder WASM binary');
            }
            finally {
                // Restore console.warn
                console.warn = originalWarn;
            }
        });
    });
});
