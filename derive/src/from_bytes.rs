use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote, Data, DeriveInput, Fields, GenericParam, Generics};

pub fn generate_from_bytes(input: DeriveInput) -> TokenStream {
    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let deserialization = generate_deserialize(&input.data);

    let type_name = format!("{}", name);
    let expanded = quote! {
        impl #impl_generics FromBytes for #name #ty_generics #where_clause {
            fn deserialize(buf:  &mut Bytes, is_flexible_version: bool, version:u16) -> Self {
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

pub fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(FromBytes));
        }
    }
    generics
}

fn generate_deserialize(data: &Data) -> quote::__private::TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let f_name = format!("{}", name.clone().unwrap());
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

                    quote_spanned! {f.span()=>
                        #name: {
                            if version >= #min_version {
                                log::trace!("Deserializing field {}",#f_name);
                                let val = FromBytes::deserialize(buf, is_flexible_version, version);
                                log::trace!("Deserialized field {} {:?}",#f_name,val);
                                log::trace!("Bytes left: {:03?}",buf.to_vec());
                                val
                            } else {
                                Default::default()
                            }
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
