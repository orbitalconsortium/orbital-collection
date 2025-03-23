export declare let dirPath: string;
/**
 * Options for generating a container
 */
export interface ContainerOptions {
    /**
     * Optional template to use instead of the default
     */
    template?: string;
    /**
     * Optional template content to use instead of loading from a file
     */
    templateContent?: string;
}
/**
 * Generate a WAT file with embedded data
 *
 * @param data The data to embed in the WAT file
 * @param options Options for generating the WAT file
 * @returns The WAT file content as a string
 */
export declare function generateWat(data: Uint8Array, options?: ContainerOptions): string;
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
export declare function generateWasm(data: Uint8Array, wat2wasm: Wat2Wasm, options?: ContainerOptions): Promise<Uint8Array>;
/**
 * Default implementation of wat2wasm using WebAssembly.validate
 * This is a placeholder that doesn't actually convert WAT to WASM
 * In a real implementation, you would use a library like wabt.js
 *
 * @param wat WAT code to convert
 * @returns Promise that resolves to a placeholder Uint8Array
 */
export declare function defaultWat2Wasm(wat: string): Promise<Uint8Array>;
/**
 * Implementation of wat2wasm using wabt.js
 * This requires the wabt.js library to be loaded
 *
 * @param wat WAT code to convert
 * @returns Promise that resolves to the WASM binary
 */
export declare function wabtWat2Wasm(wat: string): Promise<Uint8Array>;
/**
 * Browser-friendly function to generate a container WASM file
 *
 * @param file File object to embed in the WASM
 * @param wat2wasm Function to convert WAT to WASM (defaults to placeholder)
 * @param options Options for generating the WASM file
 * @returns Promise that resolves to the WASM file content as a Uint8Array
 */
export declare function generateContainerFromFile(file: File, wat2wasm?: Wat2Wasm, options?: ContainerOptions): Promise<Uint8Array>;
/**
 * Node.js-friendly function to generate a container WASM file
 *
 * @param filePath Path to the file to embed in the WASM
 * @param wat2wasm Function to convert WAT to WASM
 * @param options Options for generating the WASM file
 * @returns Promise that resolves to the WASM file content as a Uint8Array
 */
export declare function generateContainerFromFilePath(filePath: string, wat2wasm: Wat2Wasm, options?: ContainerOptions): Promise<Uint8Array>;
/**
 * Generate a container WASM file from data
 *
 * @param data The data to embed in the WASM
 * @param wat2wasm Function to convert WAT to WASM (defaults to placeholder)
 * @param options Options for generating the WASM file
 * @returns Promise that resolves to the WASM file content as a Uint8Array
 */
export declare function generateContainerFromData(data: Uint8Array, wat2wasm?: Wat2Wasm, options?: ContainerOptions): Promise<Uint8Array>;
