import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';
import { 
  generateWat, 
  generateWasm, 
  generateContainerFromData,
  defaultWat2Wasm
} from '../src/index';

describe('Container Generator Library Tests', () => {
  let tempDir: string;
  
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
      const wat = generateWat(testData);
      
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
      const wat = generateWat(testData, { templateContent: customTemplate });
      
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
      const wasm = await generateWasm(testData, mockWat2Wasm);
      
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
      const wasm = await generateContainerFromData(testData, mockWat2Wasm);
      
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
        const wasm = await defaultWat2Wasm(testWat);
        
        // Check that a warning was logged
        expect(console.warn).toHaveBeenCalled();
        
        // Check that a placeholder was returned
        expect(wasm.length).toBeGreaterThan(0);
        
        // Check that the placeholder contains the expected text
        const decoder = new TextDecoder();
        expect(decoder.decode(wasm)).toBe('Placeholder WASM binary');
      } finally {
        // Restore console.warn
        console.warn = originalWarn;
      }
    });
  });
});