extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl, Type, TypePath, DeriveInput};

/// A macro for declaring an orbital collection implementation.
///
/// This macro generates the necessary __execute and __meta functions for the WebAssembly interface
/// specifically tailored for orbital collections.
#[proc_macro]
pub fn declare_orbital_collection(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemImpl);
    
    // Extract the struct name and message type
    let struct_name = match &*input.self_ty {
        Type::Path(TypePath { path, .. }) => {
            let segments = &path.segments;
            if segments.is_empty() {
                panic!("Expected a struct name");
            }
            &segments.last().unwrap().ident
        }
        _ => panic!("Expected a struct name"),
    };
    
    // Find the message type in the impl items
    let message_type = input.items.iter().find_map(|item| {
        if let syn::ImplItem::Type(type_item) = item {
            if type_item.ident == "Message" {
                if let syn::Type::Path(type_path) = &type_item.ty {
                    return Some(&type_path.path.segments.last().unwrap().ident);
                }
            }
        }
        None
    }).unwrap_or_else(|| panic!("Expected a Message type"));
    
    // Generate the __execute and __meta functions
    let expanded = quote! {
        #[no_mangle]
        pub extern "C" fn __execute() -> i32 {
            use alkanes_runtime::runtime::{handle_error, handle_success};
            use metashrew_support::compat::to_arraybuffer_layout;

            let mut context = #struct_name::default().context().unwrap();
            let mut inputs = context.inputs.clone();

            if inputs.is_empty() {
                let extended = handle_error("No opcode provided");
                return alkanes_runtime::runtime::response_to_i32(extended);
            }

            let opcode = inputs[0];
            inputs.remove(0);

            let result = match #message_type::dispatch_opcode(opcode, inputs, &#struct_name::default()) {
                Ok(response) => Ok(response),
                Err(err) => Err(anyhow::anyhow!("Failed to dispatch message: {}", err)),
            };

            let extended = match result {
                Ok(res) => handle_success(res),
                Err(err) => {
                    let error_msg = format!("Error: {}", err);
                    let extended = handle_error(&error_msg);
                    return alkanes_runtime::runtime::response_to_i32(extended);
                }
            };

            alkanes_runtime::runtime::response_to_i32(extended)
        }

        #[no_mangle]
        pub extern "C" fn __meta() -> i32 {
            let abi = #message_type::export_abi();
            export_bytes(&abi)
        }

        fn export_bytes(data: &[u8]) -> i32 {
            let response_bytes = to_arraybuffer_layout(data);
            Box::leak(Box::new(response_bytes)).as_mut_ptr() as usize as i32 + 4
        }
    };

    TokenStream::from(expanded)
}

/// A macro for declaring an orbital instance implementation.
///
/// This macro generates the necessary __execute and __meta functions for the WebAssembly interface
/// specifically tailored for orbital instances.
#[proc_macro]
pub fn declare_orbital(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemImpl);
    
    // Extract the struct name and message type
    let struct_name = match &*input.self_ty {
        Type::Path(TypePath { path, .. }) => {
            let segments = &path.segments;
            if segments.is_empty() {
                panic!("Expected a struct name");
            }
            &segments.last().unwrap().ident
        }
        _ => panic!("Expected a struct name"),
    };
    
    // Find the message type in the impl items
    let message_type = input.items.iter().find_map(|item| {
        if let syn::ImplItem::Type(type_item) = item {
            if type_item.ident == "Message" {
                if let syn::Type::Path(type_path) = &type_item.ty {
                    return Some(&type_path.path.segments.last().unwrap().ident);
                }
            }
        }
        None
    }).unwrap_or_else(|| panic!("Expected a Message type"));
    
    // Generate the __execute and __meta functions
    let expanded = quote! {
        #[no_mangle]
        pub extern "C" fn __execute() -> i32 {
            use alkanes_runtime::runtime::{handle_error, handle_success};
            use metashrew_support::compat::to_arraybuffer_layout;

            let mut context = #struct_name::default().context().unwrap();
            let mut inputs = context.inputs.clone();

            if inputs.is_empty() {
                let extended = handle_error("No opcode provided");
                return alkanes_runtime::runtime::response_to_i32(extended);
            }

            let opcode = inputs[0];
            inputs.remove(0);

            let result = match #message_type::dispatch_opcode(opcode, inputs, &#struct_name::default()) {
                Ok(response) => Ok(response),
                Err(err) => Err(anyhow::anyhow!("Failed to dispatch message: {}", err)),
            };

            let extended = match result {
                Ok(res) => handle_success(res),
                Err(err) => {
                    let error_msg = format!("Error: {}", err);
                    let extended = handle_error(&error_msg);
                    return alkanes_runtime::runtime::response_to_i32(extended);
                }
            };

            alkanes_runtime::runtime::response_to_i32(extended)
        }

        #[no_mangle]
        pub extern "C" fn __meta() -> i32 {
            let abi = #message_type::export_abi();
            export_bytes(&abi)
        }

        fn export_bytes(data: &[u8]) -> i32 {
            let response_bytes = to_arraybuffer_layout(data);
            Box::leak(Box::new(response_bytes)).as_mut_ptr() as usize as i32 + 4
        }
    };

    TokenStream::from(expanded)
}

/// A derive macro for orbital collection messages.
///
/// This macro generates the implementation of the MessageDispatch trait for orbital collection messages.
#[proc_macro_derive(OrbitalCollectionMessage, attributes(opcode, returns))]
pub fn derive_orbital_collection_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Extract the enum name
    let enum_name = &input.ident;
    
    // Generate the implementation
    let expanded = quote! {
        impl #enum_name {
            pub fn from_opcode(opcode: u128, inputs: Vec<u128>) -> Result<Self, anyhow::Error> {
                use orbital_traits::collection_opcodes::*;
                match opcode {
                    INITIALIZE => {
                        if inputs.len() >= 3 {
                            Ok(Self::Initialize {
                                name_part1: inputs[0],
                                name_part2: inputs[1],
                                symbol: inputs[2],
                            })
                        } else {
                            Err(anyhow::anyhow!("Missing parameters for Initialize"))
                        }
                    },
                    CREATE_ORBITAL => Ok(Self::CreateOrbital),
                    GET_NAME => Ok(Self::GetName),
                    GET_SYMBOL => Ok(Self::GetSymbol),
                    GET_TOTAL_SUPPLY => Ok(Self::GetTotalSupply),
                    GET_ORBITAL_COUNT => Ok(Self::GetOrbitalCount),
                    GET_DATA => Ok(Self::GetData),
                    _ => Err(anyhow::anyhow!("Unknown opcode: {}", opcode)),
                }
            }
            
            pub fn dispatch(&self, responder: &BitcoinCollection) -> Result<alkanes_support::response::CallResponse, anyhow::Error> {
                match self {
                    Self::Initialize { name_part1, name_part2, symbol } => {
                        responder.initialize(*name_part1, *name_part2, *symbol)
                    },
                    Self::CreateOrbital => responder.create_orbital(),
                    Self::GetName => responder.get_name(),
                    Self::GetSymbol => responder.get_symbol(),
                    Self::GetTotalSupply => responder.get_total_supply(),
                    Self::GetOrbitalCount => responder.get_orbital_count(),
                    Self::GetData => responder.get_data(),
                }
            }
            
            pub fn dispatch_opcode(opcode: u128, inputs: Vec<u128>, responder: &BitcoinCollection) -> Result<alkanes_support::response::CallResponse, anyhow::Error> {
                let message = Self::from_opcode(opcode, inputs)?;
                message.dispatch(responder)
            }
            
            pub fn export_abi() -> Vec<u8> {
                r#"[
                    {"opcode":0,"name":"Initialize","inputs":[{"name":"name_part1","type":"u128"},{"name":"name_part2","type":"u128"},{"name":"symbol","type":"u128"}],"outputs":[]},
                    {"opcode":77,"name":"CreateOrbital","inputs":[],"outputs":[]},
                    {"opcode":99,"name":"GetName","inputs":[],"outputs":[{"type":"String"}]},
                    {"opcode":100,"name":"GetSymbol","inputs":[],"outputs":[{"type":"String"}]},
                    {"opcode":101,"name":"GetTotalSupply","inputs":[],"outputs":[{"type":"u128"}]},
                    {"opcode":102,"name":"GetOrbitalCount","inputs":[],"outputs":[{"type":"u128"}]},
                    {"opcode":1000,"name":"GetData","inputs":[],"outputs":[{"type":"Vec<u8>"}]}
                ]"#.as_bytes().to_vec()
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// A derive macro for orbital instance messages.
///
/// This macro generates the implementation of the MessageDispatch trait for orbital instance messages.
#[proc_macro_derive(OrbitalInstanceMessage, attributes(opcode, returns))]
pub fn derive_orbital_instance_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Extract the enum name
    let enum_name = &input.ident;
    
    // Generate the implementation
    let expanded = quote! {
        impl #enum_name {
            pub fn from_opcode(opcode: u128, inputs: Vec<u128>) -> Result<Self, anyhow::Error> {
                use orbital_traits::orbital_opcodes::*;
                match opcode {
                    INITIALIZE => {
                        if inputs.len() >= 1 {
                            Ok(Self::Initialize { index: inputs[0] })
                        } else {
                            Err(anyhow::anyhow!("Missing index parameter for Initialize"))
                        }
                    },
                    GET_NAME => Ok(Self::GetName),
                    GET_SYMBOL => Ok(Self::GetSymbol),
                    GET_TOTAL_SUPPLY => Ok(Self::GetTotalSupply),
                    GET_DATA => Ok(Self::GetData),
                    _ => Err(anyhow::anyhow!("Unknown opcode: {}", opcode)),
                }
            }
            
            pub fn dispatch<T>(&self, responder: &T) -> Result<alkanes_support::response::CallResponse, anyhow::Error> 
            where 
                T: orbital_traits::OrbitalInstance
            {
                match self {
                    Self::Initialize { index } => responder.initialize(*index),
                    Self::GetName => responder.get_name(),
                    Self::GetSymbol => responder.get_symbol(),
                    Self::GetTotalSupply => responder.get_total_supply(),
                    Self::GetData => responder.get_data(),
                }
            }
            
            pub fn dispatch_opcode<T>(opcode: u128, inputs: Vec<u128>, responder: &T) -> Result<alkanes_support::response::CallResponse, anyhow::Error>
            where
                T: orbital_traits::OrbitalInstance
            {
                let message = Self::from_opcode(opcode, inputs)?;
                message.dispatch(responder)
            }
            
            pub fn export_abi() -> Vec<u8> {
                r#"[
                    {"opcode":0,"name":"Initialize","inputs":[{"name":"index","type":"u128"}],"outputs":[]},
                    {"opcode":99,"name":"GetName","inputs":[],"outputs":[{"type":"String"}]},
                    {"opcode":100,"name":"GetSymbol","inputs":[],"outputs":[{"type":"String"}]},
                    {"opcode":101,"name":"GetTotalSupply","inputs":[],"outputs":[{"type":"u128"}]},
                    {"opcode":1000,"name":"GetData","inputs":[],"outputs":[{"type":"Vec<u8>"}]}
                ]"#.as_bytes().to_vec()
            }
        }
    };
    
    TokenStream::from(expanded)
}