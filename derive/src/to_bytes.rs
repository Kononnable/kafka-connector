use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote, Data, DeriveInput, Fields, GenericParam, Generics};

pub fn generate_to_bytes(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let serialization = generate_serialize(&input.data);

    let expanded = quote! {
        impl #impl_generics ToBytes for #name #ty_generics #where_clause {
            fn serialize(&self, buf:&mut bytes::BytesMut, is_flexible_version: bool, version:u16) {
                log::trace!("Serializing {:#?}", self);
                #serialization
                log::trace!("Serialization finished {:#?}", self);
            }
        }
    };
    TokenStream::from(expanded)
}

pub fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(ToBytes));
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
                    let min_version = f
                        .attrs
                        .iter()
                        .find(|a| a.path.segments.last().unwrap().ident == "min_version")
                        .map(|x| {
                            x.tokens
                                .clone()
                                .into_iter()
                                .last()
                                .unwrap()
                                .to_string()
                                .parse::<u16>()
                                .unwrap()
                        })
                        .unwrap_or_default();

                    let f_name = format!("{}", name.clone().unwrap());
                    quote_spanned! {f.span()=>
                        if version >= #min_version {
                            ToBytes::serialize(&self.#name,buf, is_flexible_version,version);
                            log::trace!("Serialized field {}",#f_name);
                            log::trace!("Bytes: {:03?}",buf.to_vec());
                        }
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
