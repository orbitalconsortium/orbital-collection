use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use anyhow::{anyhow, Result};

/// Generate a container WASM file from a data file
///
/// This function takes a data file path and generates a WASM container
/// that will return the data when the GetData opcode (1000) is called.
///
/// # Arguments
///
/// * `data_path` - Path to the data file
/// * `output_path` - Path where the WASM file will be saved
///
/// # Returns
///
/// * `Result<()>` - Ok if successful, Err otherwise
pub fn generate_container_from_file<P: AsRef<Path>>(data_path: P, output_path: P) -> Result<()> {
    // Check if the data file exists
    if !data_path.as_ref().exists() {
        return Err(anyhow!("Data file not found: {:?}", data_path.as_ref()));
    }

    // Get the path to the script
    let script_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("generate_container.sh");
    
    // Check if the script exists
    if !script_path.exists() {
        return Err(anyhow!("Script not found: {:?}", script_path));
    }
    
    // Make the script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }
    
    // Run the script
    let status = Command::new(&script_path)
        .arg(data_path.as_ref())
        .arg(output_path.as_ref())
        .status()?;
    
    if !status.success() {
        return Err(anyhow!("Failed to generate container WASM"));
    }
    
    Ok(())
}

/// Generate a container WASM file from raw data
///
/// This function takes raw data and generates a WASM container
/// that will return the data when the GetData opcode (1000) is called.
///
/// # Arguments
///
/// * `data` - The raw data bytes
/// * `output_path` - Path where the WASM file will be saved
///
/// # Returns
///
/// * `Result<()>` - Ok if successful, Err otherwise
pub fn generate_container_from_data<P: AsRef<Path>>(data: &[u8], output_path: P) -> Result<()> {
    // Create a temporary file for the data
    let temp_dir = tempfile::tempdir()?;
    let temp_file_path = temp_dir.path().join("temp_data");
    
    // Write the data to the temporary file
    let mut temp_file = fs::File::create(&temp_file_path)?;
    temp_file.write_all(data)?;
    temp_file.flush()?;
    
    // Generate the container using the temporary file
    generate_container_from_file(&temp_file_path, output_path)?;
    
    // The temporary directory will be automatically cleaned up when it goes out of scope
    
    Ok(())
}

/// Generate a WAT file with embedded data
///
/// This function takes raw data and generates a WAT file with the data embedded.
///
/// # Arguments
///
/// * `data` - The raw data bytes
/// * `output_path` - Path where the WAT file will be saved
///
/// # Returns
///
/// * `Result<()>` - Ok if successful, Err otherwise
pub fn generate_wat_with_data<P: AsRef<Path>>(data: &[u8], output_path: P) -> Result<()> {
    // Get the path to the template WAT file
    let template_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("template.wat");
    
    // Check if the template exists
    if !template_path.exists() {
        return Err(anyhow!("Template WAT file not found: {:?}", template_path));
    }
    
    // Read the template
    let template = fs::read_to_string(&template_path)?;
    
    // Convert data to hex string
    let mut hex_data = String::new();
    for byte in data {
        hex_data.push_str(&format!("\\{:02x}", byte));
    }
    
    // Replace placeholders in the template
    let wat_content = template
        .replace("DATA_PLACEHOLDER", &hex_data)
        .replace("DATA_SIZE", &data.len().to_string());
    
    // Write the WAT file
    fs::write(output_path, wat_content)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_generate_wat_with_data() {
        // Create a temporary directory for the test
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("test.wat");
        
        // Generate a WAT file with some test data
        let test_data = b"Hello, World!";
        generate_wat_with_data(test_data, &output_path).unwrap();
        
        // Check if the WAT file was created
        assert!(output_path.exists());
        
        // Read the WAT file
        let wat_content = fs::read_to_string(&output_path).unwrap();
        
        // Check if the data was embedded correctly
        assert!(wat_content.contains("\\48\\65\\6c\\6c\\6f\\2c\\20\\57\\6f\\72\\6c\\64\\21"));
        assert!(wat_content.contains("DATA_SIZE"));
        assert!(wat_content.contains(&test_data.len().to_string()));
    }
}