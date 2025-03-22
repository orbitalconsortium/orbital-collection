#!/bin/bash

# Check if a file path is provided
if [ $# -lt 1 ]; then
  echo "Usage: $0 <file_path> [output_wasm_path]"
  echo "Example: $0 image.png container.wasm"
  exit 1
fi

# Get the input file path
INPUT_FILE="$1"

# Set the output WASM path (default or provided)
if [ $# -ge 2 ]; then
  OUTPUT_WASM="$2"
else
  OUTPUT_WASM="container.wasm"
fi

# Set the template WAT path
TEMPLATE_WAT="$(dirname "$0")/template.wat"
TEMP_WAT="$(dirname "$0")/temp.wat"

# Check if the input file exists
if [ ! -f "$INPUT_FILE" ]; then
  echo "Error: Input file '$INPUT_FILE' not found."
  exit 1
fi

# Check if the template WAT file exists
if [ ! -f "$TEMPLATE_WAT" ]; then
  echo "Error: Template WAT file '$TEMPLATE_WAT' not found."
  exit 1
fi

# Check if wat2wasm is installed
if ! command -v wat2wasm &> /dev/null; then
  echo "Error: wat2wasm not found. Please install the WebAssembly Binary Toolkit (WABT)."
  echo "You can install it using:"
  echo "  - On macOS: brew install wabt"
  echo "  - On Ubuntu/Debian: apt-get install wabt"
  echo "  - On other systems: https://github.com/WebAssembly/wabt"
  exit 1
fi

echo "Generating container WASM from file: $INPUT_FILE"

# Get the file size
FILE_SIZE=$(wc -c < "$INPUT_FILE")
echo "File size: $FILE_SIZE bytes"

# Convert the file to a hex string
HEX_DATA=$(xxd -p "$INPUT_FILE" | tr -d '\n')
echo "Converted file to hex string"

# Format the hex string as a WAT data string
WAT_DATA=""
for (( i=0; i<${#HEX_DATA}; i+=2 )); do
  BYTE="${HEX_DATA:$i:2}"
  WAT_DATA+="\\$BYTE"
done

echo "Creating WAT file with embedded data"

# Replace the placeholders in the template
sed "s/DATA_PLACEHOLDER/$WAT_DATA/g" "$TEMPLATE_WAT" > "$TEMP_WAT"
sed -i "s/DATA_SIZE/$FILE_SIZE/g" "$TEMP_WAT"

echo "Compiling WAT to WASM"

# Compile the WAT to WASM
wat2wasm "$TEMP_WAT" -o "$OUTPUT_WASM"

# Check if compilation was successful
if [ $? -eq 0 ]; then
  echo "Successfully generated WASM file: $OUTPUT_WASM"
  # Clean up the temporary WAT file
  rm "$TEMP_WAT"
else
  echo "Error: Failed to compile WAT to WASM."
  exit 1
fi

# Print the size of the generated WASM file
WASM_SIZE=$(wc -c < "$OUTPUT_WASM")
echo "Generated WASM size: $WASM_SIZE bytes"

echo "Done!"