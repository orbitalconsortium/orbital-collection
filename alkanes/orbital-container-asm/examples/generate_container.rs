use orbital_container_asm::{generate_container_from_file, generate_container_from_data};
use std::env;
use std::path::Path;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    match args[1].as_str() {
        "file" => {
            if args.len() < 4 {
                println!("Error: Missing arguments for 'file' command");
                print_usage();
                return;
            }
            
            let input_path = &args[2];
            let output_path = &args[3];
            
            println!("Generating container from file: {}", input_path);
            println!("Output path: {}", output_path);
            
            match generate_container_from_file(input_path, output_path) {
                Ok(_) => println!("Container generated successfully!"),
                Err(e) => println!("Error: {}", e),
            }
        },
        "data" => {
            if args.len() < 4 {
                println!("Error: Missing arguments for 'data' command");
                print_usage();
                return;
            }
            
            let data = args[2].as_bytes();
            let output_path = &args[3];
            
            println!("Generating container from data: {} bytes", data.len());
            println!("Output path: {}", output_path);
            
            match generate_container_from_data(data, output_path) {
                Ok(_) => println!("Container generated successfully!"),
                Err(e) => println!("Error: {}", e),
            }
        },
        "help" => {
            print_usage();
        },
        _ => {
            println!("Error: Unknown command '{}'", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  generate_container file <input_path> <output_path>");
    println!("  generate_container data <text_data> <output_path>");
    println!("  generate_container help");
    println!();
    println!("Examples:");
    println!("  generate_container file image.png container.wasm");
    println!("  generate_container data \"Hello, World!\" container.wasm");
}