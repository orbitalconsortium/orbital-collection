# Orbitals Support

A support library for implementing orbital alkanes in the alkane framework. This library provides traits and utilities for creating orbital instances that can transform data based on their index and sequence number.

## Overview

The orbitals-support library provides:

1. The `Orbital` trait - A trait that defines the common functionality for orbital alkanes
2. The `BytesTransform` trait - A trait for transforming data bytes based on index and sequence
3. Example implementations - Examples of how to implement custom transforms and orbitals

## Usage

### Basic Usage

To create a basic orbital alkane that uses the default implementation:

```rust
use alkanes_runtime::{runtime::AlkaneResponder, token::Token};
use alkanes_support::{parcel::AlkaneTransfer, response::CallResponse};
use orbitals_support::{Orbital, BytesTransform, IdentityTransform};

#[derive(Default)]
pub struct MyOrbital(());

impl Token for MyOrbital {
    fn name(&self) -> String {
        self.default_name()
    }
    
    fn symbol(&self) -> String {
        self.default_symbol()
    }
}

impl Orbital for MyOrbital {
    fn get_transform(&self) -> Box<dyn BytesTransform> {
        // Use the identity transform (no transformation)
        Box::new(IdentityTransform)
    }
}
```

### Custom Transforms

To create a custom transform that modifies the data:

```rust
use orbitals_support::BytesTransform;

pub struct MyCustomTransform;

impl BytesTransform for MyCustomTransform {
    fn transform(&self, input: &[u8], index: u128, sequence: u128) -> Vec<u8> {
        // Apply your custom transformation here
        // For example, if working with images:
        // 1. Parse the input bytes as an image
        // 2. Apply transformations based on the index and sequence
        // 3. Encode the transformed image back to bytes
        
        // For now, just return the input bytes unchanged
        input.to_vec()
    }
}

// Then in your orbital implementation:
impl Orbital for MyOrbital {
    fn get_transform(&self) -> Box<dyn BytesTransform> {
        Box::new(MyCustomTransform)
    }
}
```

### Image Transformations

The library is designed to work with image libraries like `image` for transforming image data. Here's an example of how you might implement an image transform:

```rust
use orbitals_support::BytesTransform;
use image::{DynamicImage, ImageFormat};

pub struct ImageTransform;

impl BytesTransform for ImageTransform {
    fn transform(&self, input: &[u8], index: u128, _sequence: u128) -> Vec<u8> {
        // Parse the input bytes as an image
        let img = image::load_from_memory(input).unwrap();
        
        // Apply transformations based on the index
        let transformed = match index % 4 {
            0 => img.grayscale(),
            1 => img.rotate90(),
            2 => img.fliph(),
            3 => img.flipv(),
            _ => img,
        };
        
        // Encode the transformed image back to bytes
        let mut buffer = Vec::new();
        transformed.write_to(&mut buffer, ImageFormat::Png).unwrap();
        buffer
    }
}
```

## Examples

The library includes several examples:

1. `examples.rs` - Contains example implementations of `BytesTransform`
2. `custom_orbital_example.rs` - Contains an example of a custom orbital implementation

## Orbital Trait

The `Orbital` trait provides default implementations for most of the functionality needed by an orbital alkane:

- Getting and setting the collection alkane ID
- Getting and setting the index
- Getting the sequence number
- Getting the collection's name and symbol
- Converting numbers to superscript
- Getting and setting the total supply
- Observing initialization
- Getting the data with transformation

The only method you need to implement is `get_transform()`, which returns the transform to apply to the data.

## BytesTransform Trait

The `BytesTransform` trait defines a single method:

```rust
fn transform(&self, input: &[u8], index: u128, sequence: u128) -> Vec<u8>;
```

This method takes the input bytes, the index of the orbital in the collection, and the sequence number of the orbital, and returns the transformed bytes.

## License

This project is licensed under the MIT License - see the LICENSE file for details.