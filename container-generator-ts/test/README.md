# Container Generator Test Suite

This directory contains tests for the orbitals-container-generator package.

## Test Files

- `cli.test.ts`: Tests for the CLI interface using child_process to spawn the CLI process
- `index.test.ts`: Tests for the library functions

## Running Tests

To run the tests, use the following commands:

```bash
# Install dependencies
npm install

# Build the project
npm run build

# Run tests
npm test

# Run tests with coverage
npm run test:coverage

# Run tests in watch mode
npm run test:watch
```

## CLI Tests

The CLI tests use child_process to spawn the CLI process and test its functionality. The tests:

1. Generate random PNG files in a temporary directory
2. Invoke the CLI process to generate WASM containers
3. Verify the output WASM files

The tests cover:
- Basic functionality (generating a WASM file from a PNG file)
- Error handling (non-existent input file)
- Default output file name
- Custom template
- Large file handling

## Library Tests

The library tests test the core functionality of the package:

- `generateWat`: Generates a WAT file with embedded data
- `generateWasm`: Converts a WAT file to a WASM file
- `generateContainerFromData`: Generates a container from raw data
- `defaultWat2Wasm`: Placeholder implementation for WAT to WASM conversion

## Test Coverage

The tests aim to cover all the main functionality of the package, including:

- Input validation
- Error handling
- File generation
- Template customization
- Large file handling

## Adding New Tests

To add new tests:

1. Create a new test file in this directory
2. Import the necessary functions from the package
3. Write your tests using Jest
4. Run the tests to ensure they pass