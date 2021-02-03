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
            fn serialize(&self, buf:&mut bytes::BytesMut, is_flexible_version: bool) {
                log::trace!("Serializing {:#?}", self);
                #serialization
                log::trace!("Serialization finished {:#?}", self);
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
                    let f_name = format!("{}", name.clone().unwrap());
                    quote_spanned! {f.span()=>
                        ToBytes::serialize(&self.#name,buf, is_flexible_version);
                        log::trace!("Serialized field {}",#f_name);
                        log::trace!("Bytes: {:03?}",buf.to_vec());
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
            fn deserialize(buf:  &mut Bytes, is_flexible_version: bool) -> Self {
                log::trace!("Deserializing {}",#type_name);
                let ret_val = #name {
                    #deserialization
                };
                log::trace!("Deserialization finished \n{:#?}", ret_val);
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
                            log::trace!("Deserializing field {}",#f_name);
                            let val = FromBytes::deserialize(buf, is_flexible_version);
                            log::trace!("Deserialized field {} {:?}",#f_name,val);
                            log::trace!("Bytes left: {:03?}",buf.to_vec());
                            val
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
