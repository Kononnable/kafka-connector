use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics};

#[proc_macro_derive(ToBytes)]
pub fn to_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let serialization = generate_serialize(&input.data);

    let expanded = quote! {
        impl #impl_generics ToBytes for #name #ty_generics #where_clause {
            fn serialize(&self, buf:&mut bytes::BytesMut) {
                trace!("Serializing {:#?}", self);
                #serialization
                trace!("Serialization finished {:#?}", self);
            }
        }
    };
    TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(FromBytes));
        }
    }
    generics
}

fn generate_serialize(data: &Data) -> quote::__private::TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {f.span()=>
                        ToBytes::serialize(&self.#name,buf);
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            _ => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

#[proc_macro_derive(FromBytes)]
pub fn from_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let deserialization = generate_deserialize(&input.data);
    let type_name = format!("{}", name);
    let expanded = quote! {
        impl #impl_generics FromBytes for #name #ty_generics #where_clause {
            fn deserialize(buf:  &mut Bytes) -> Self {
                trace!("Deserializing {}",#type_name);
                let ret_val = #name {
                    #deserialization
                };
                trace!("Deserialization finished {:#?}", ret_val);
                ret_val
            }
        }
    };
    TokenStream::from(expanded)
}

fn generate_deserialize(data: &Data) -> quote::__private::TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let f_name = format!("{}", name.clone().unwrap());
                    quote_spanned! {f.span()=>
                        #name: {
                            trace!("Deserializing field {}",#f_name);
                            FromBytes::deserialize(buf)
                        },
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            _ => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
