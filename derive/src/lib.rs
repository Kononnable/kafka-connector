use from_bytes::generate_from_bytes;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use to_bytes::generate_to_bytes;

mod from_bytes;
mod to_bytes;

#[proc_macro_derive(ToBytes, attributes(min_version))]
pub fn to_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    generate_to_bytes(input)
}

#[proc_macro_derive(FromBytes, attributes(min_version))]
pub fn from_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    generate_from_bytes(input)
}
