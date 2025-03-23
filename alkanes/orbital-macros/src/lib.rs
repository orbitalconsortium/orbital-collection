extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl, Type, TypePath};

/// A macro for declaring an orbital alkane implementation.
///
/// This macro is similar to the declare_alkane! macro but specifically tailored for orbital alkanes.
/// It generates the necessary __execute and __meta functions for the WebAssembly interface.
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
            use alkanes_runtime::runtime::AlkaneResponder;
            use alkanes_runtime::runtime::{handle_error, handle_success, prepare_response};
            use metashrew_support::compat::{to_arraybuffer_layout, to_passback_ptr};

            let mut context = #struct_name::default().context().unwrap();
            let mut inputs = context.inputs.clone();

            if inputs.is_empty() {
                let extended = handle_error("No opcode provided");
                return alkanes_runtime::runtime::response_to_i32(extended);
            }

            let opcode = inputs[0];
            inputs.remove(0);

            let result = match #message_type::from_opcode(opcode, inputs) {
                Ok(message) => message.dispatch(&#struct_name::default()),
                Err(err) => Err(anyhow::anyhow!("Failed to parse message: {}", err)),
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

/// A macro for defining message dispatch for orbital alkanes.
///
/// This macro is similar to the MessageDispatch derive macro but specifically tailored for orbital alkanes.
#[proc_macro_derive(OrbitalMessage, attributes(opcode, returns))]
pub fn derive_orbital_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    
    // Extract the enum name
    let enum_name = &input.ident;
    
    // Generate the implementation
    let expanded = quote! {
        impl alkanes_runtime::message::MessageDispatch<OrbitalInstance> for #enum_name {
            fn from_opcode(opcode: u128, inputs: Vec<u128>) -> Result<Self, anyhow::Error> {
                match opcode {
                    0 => {
                        if inputs.len() >= 1 {
                            Ok(Self::Initialize { index: inputs[0] })
                        } else {
                            Err(anyhow::anyhow!("Missing index parameter for Initialize"))
                        }
                    },
                    99 => Ok(Self::GetName),
                    100 => Ok(Self::GetSymbol),
                    101 => Ok(Self::GetTotalSupply),
                    1000 => Ok(Self::GetData),
                    _ => Err(anyhow::anyhow!("Unknown opcode: {}", opcode)),
                }
            }
            
            fn dispatch(&self, responder: &OrbitalInstance) -> Result<alkanes_support::response::CallResponse, anyhow::Error> {
                match self {
                    Self::Initialize { index } => responder.initialize(*index),
                    Self::GetName => responder.get_name(),
                    Self::GetSymbol => responder.get_symbol(),
                    Self::GetTotalSupply => responder.get_total_supply(),
                    Self::GetData => responder.get_data(),
                }
            }
            
            fn export_abi() -> Vec<u8> {
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