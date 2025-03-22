/**
 * Template for the WebAssembly Text Format (WAT) file
 * This template will be used to generate the container WASM
 */
export const WAT_TEMPLATE = `(module
  ;; Memory declaration
  (memory (export "memory") 1)

  ;; Import the abort function
  (import "env" "abort" (func $abort (param i32 i32 i32 i32)))

  ;; Define our data section - this will be replaced
  (data (i32.const 0) "DATA_PLACEHOLDER")

  ;; Export the __execute function
  (func (export "__execute") (result i32)
    ;; Create a CallResponse with empty alkanes and our data
    ;; Format of CallResponse: [alkanes_count(16 bytes)][data]
    
    ;; First, we need to create the CallResponse
    ;; We'll allocate memory at position 1024 for our CallResponse
    (i32.const 1024)              ;; Destination address for CallResponse
    
    ;; Store alkanes count (0) as first 16 bytes
    (i64.const 0)                 ;; No alkanes (first 8 bytes)
    (i64.store)                   ;; Store at position 1024
    
    (i32.const 1024)
    (i32.const 8)
    (i32.add)                     ;; Position 1024 + 8
    (i64.const 0)                 ;; No alkanes (second 8 bytes)
    (i64.store)                   ;; Store at position 1024 + 8
    
    ;; Now copy our data after the alkanes count
    (i32.const 1024)              ;; Destination base
    (i32.const 16)                ;; Destination offset (after alkanes count)
    (i32.add)                     ;; Destination address
    (i32.const 0)                 ;; Source address (our data)
    (i32.const DATA_SIZE)         ;; Size to copy - will be replaced
    (memory.copy)                 ;; Copy the data
    
    ;; Now we need to create the arraybuffer layout
    ;; Format: [size(4 bytes)][data]
    ;; Where data is our CallResponse: [alkanes_count(16 bytes)][data]
    
    ;; Calculate the total size of our CallResponse
    (i32.const 16)                ;; 16 bytes for alkanes count
    (i32.const DATA_SIZE)         ;; Size of our data - will be replaced
    (i32.add)                     ;; Total size of CallResponse
    
    ;; Allocate memory at position 2048 for our arraybuffer layout
    (i32.const 2048)              ;; Destination address for arraybuffer layout
    (local.tee 0)                 ;; Save the address in local 0
    
    ;; Store the size of our CallResponse as a 4-byte little-endian u32
    (i32.const 16)                ;; 16 bytes for alkanes count
    (i32.const DATA_SIZE)         ;; Size of our data - will be replaced
    (i32.add)                     ;; Total size of CallResponse
    (i32.store)                   ;; Store at position 2048
    
    ;; Now copy our CallResponse after the size
    (i32.const 2048)              ;; Destination base
    (i32.const 4)                 ;; Destination offset (after size)
    (i32.add)                     ;; Destination address
    (i32.const 1024)              ;; Source address (our CallResponse)
    (i32.const 16)                ;; 16 bytes for alkanes count
    (i32.const DATA_SIZE)         ;; Size of our data - will be replaced
    (i32.add)                     ;; Total size to copy
    (memory.copy)                 ;; Copy the data
    
    ;; Return the pointer to the arraybuffer layout + 4
    ;; (the +4 is because the runtime expects the pointer to point after the size)
    (local.get 0)                 ;; Get the base address
    (i32.const 4)                 ;; Offset
    (i32.add)                     ;; Add the offset
  )
)`;

/**
 * Options for generating a container
 */
export interface ContainerOptions {
  /**
   * Optional template to use instead of the default
   */
  template?: string;
}

/**
 * Generate a WAT file with embedded data
 * 
 * @param data The data to embed in the WAT file
 * @param options Options for generating the WAT file
 * @returns The WAT file content as a string
 */
export function generateWat(data: Uint8Array, options: ContainerOptions = {}): string {
  const template = options.template || WAT_TEMPLATE;
  
  // Convert data to hex string
  let hexData = '';
  for (let i = 0; i < data.length; i++) {
    const byte = data[i].toString(16).padStart(2, '0');
    hexData += `\\${byte}`;
  }
  
  // Replace placeholders in the template
  return template
    .replace('DATA_PLACEHOLDER', hexData)
    .replace(/DATA_SIZE/g, data.length.toString());
}

/**
 * Interface for the wat2wasm function
 */
export interface Wat2Wasm {
  (wat: string): Promise<Uint8Array>;
}

/**
 * Generate a WASM file with embedded data
 * 
 * @param data The data to embed in the WASM file
 * @param wat2wasm Function to convert WAT to WASM
 * @param options Options for generating the WASM file
 * @returns Promise that resolves to the WASM file content as a Uint8Array
 */
export async function generateWasm(
  data: Uint8Array,
  wat2wasm: Wat2Wasm,
  options: ContainerOptions = {}
): Promise<Uint8Array> {
  // Generate the WAT file
  const wat = generateWat(data, options);
  
  // Convert WAT to WASM using the provided function
  return await wat2wasm(wat);
}

/**
 * Default implementation of wat2wasm using WebAssembly.validate
 * This is a placeholder that doesn't actually convert WAT to WASM
 * In a real implementation, you would use a library like wabt.js
 * 
 * @param wat WAT code to convert
 * @returns Promise that resolves to a placeholder Uint8Array
 */
export async function defaultWat2Wasm(wat: string): Promise<Uint8Array> {
  // This is a placeholder implementation
  // In a real implementation, you would use a library like wabt.js
  console.warn('Using placeholder wat2wasm implementation. The generated WASM will not be valid.');
  
  // Return a placeholder Uint8Array
  const encoder = new TextEncoder();
  return encoder.encode('Placeholder WASM binary');
}

/**
 * Implementation of wat2wasm using wabt.js
 * This requires the wabt.js library to be loaded
 * 
 * @param wat WAT code to convert
 * @returns Promise that resolves to the WASM binary
 */
export async function wabtWat2Wasm(wat: string): Promise<Uint8Array> {
  // Check if wabt is available
  if (typeof (window as any).wabt === 'undefined') {
    throw new Error('wabt.js is not loaded. Please load wabt.js before calling this function.');
  }
  
  // Get the wabt instance
  const wabt = (window as any).wabt;
  
  // Wait for wabt to be ready
  await wabt.ready;
  
  // Parse the WAT code
  const module = wabt.parseWat('container.wat', wat);
  
  // Convert to binary
  const { buffer } = module.toBinary({});
  
  // Clean up
  module.destroy();
  
  return new Uint8Array(buffer);
}

/**
 * Browser-friendly function to generate a container WASM file
 * 
 * @param file File object to embed in the WASM
 * @param wat2wasm Function to convert WAT to WASM (defaults to placeholder)
 * @param options Options for generating the WASM file
 * @returns Promise that resolves to the WASM file content as a Uint8Array
 */
export async function generateContainerFromFile(
  file: File,
  wat2wasm: Wat2Wasm = defaultWat2Wasm,
  options: ContainerOptions = {}
): Promise<Uint8Array> {
  // Read the file
  const data = await readFileAsArrayBuffer(file);
  
  // Generate the WASM
  return await generateWasm(new Uint8Array(data), wat2wasm, options);
}

/**
 * Browser-friendly function to generate a container WASM file from data
 * 
 * @param data The data to embed in the WASM
 * @param wat2wasm Function to convert WAT to WASM (defaults to placeholder)
 * @param options Options for generating the WASM file
 * @returns Promise that resolves to the WASM file content as a Uint8Array
 */
export async function generateContainerFromData(
  data: Uint8Array,
  wat2wasm: Wat2Wasm = defaultWat2Wasm,
  options: ContainerOptions = {}
): Promise<Uint8Array> {
  // Generate the WASM
  return await generateWasm(data, wat2wasm, options);
}

/**
 * Helper function to read a file as an ArrayBuffer
 * 
 * @param file File to read
 * @returns Promise that resolves to the file content as an ArrayBuffer
 */
function readFileAsArrayBuffer(file: File): Promise<ArrayBuffer> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    
    reader.onload = () => {
      if (reader.result instanceof ArrayBuffer) {
        resolve(reader.result);
      } else {
        reject(new Error('Failed to read file as ArrayBuffer'));
      }
    };
    
    reader.onerror = () => {
      reject(new Error('Failed to read file'));
    };
    
    reader.readAsArrayBuffer(file);
  });
}