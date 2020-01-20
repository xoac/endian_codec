extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics,
    TypeParamBound,
};

mod attr;

#[derive(Clone, Copy)]
enum Endian {
    Big,
    Little,
    Mixed,
    Native,
}

#[derive(Clone, Copy)]
enum SerDe {
    Serialize,
    Deserialize,
}

#[proc_macro_derive(EndianSize)]
pub fn derive_endian_size(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: LittleEndianSerialize` to every type parameter T.
    let generics = add_trait_bounds(input.generics, parse_quote!(EndianSize));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = bytes_size(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics EndianSize for #name #ty_generics #where_clause {
          const BYTES_LEN: usize = #body;
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

fn bytes_size(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to an expression like
                    //
                    //     0 + <self.x as EndianSize>::BYTES_LEN + <self.y as EndianSize>::BYTES_LEN
                    let recurse = fields.named.iter().map(|f| {
                        let ty = &f.ty;
                        quote_spanned! {f.span()=>
                            <#ty as EndianSize>::BYTES_LEN
                        }
                    });

                    quote! {
                        0  #(+ #recurse)*
                    }
                }
                Fields::Unnamed(ref fields) => {
                    // Expands to an expression like
                    //
                    //     0 + <self.0 as EndianSize>::BYTES_LEN + <self.1 as EndianSize>::BYTES_LEN
                    let recurse = fields.unnamed.iter().map(|f| {
                        let ty = &f.ty;
                        quote_spanned! {f.span()=>
                            <#ty as EndianSize>::BYTES_LEN
                        }
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unit => {
                    // Unit structs cannot own more than 0 bytes of heap memory.
                    quote!(0)
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

#[proc_macro_derive(LittleEndianSerialize)]
pub fn derive_endian_ser_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_endian_impl(input, Endian::Little, SerDe::Serialize)
}

#[proc_macro_derive(BigEndianSerialize)]
pub fn derive_endian_de_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_endian_impl(input, Endian::Big, SerDe::Serialize)
}

#[proc_macro_derive(MixedEndianSerialize, attributes(endian))]
pub fn derive_endian_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_endian_impl(input, Endian::Mixed, SerDe::Serialize)
}

#[proc_macro_derive(LittleEndianDeserialize)]
pub fn derive_endian_le_de_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_endian_impl(input, Endian::Little, SerDe::Deserialize)
}

#[proc_macro_derive(BigEndianDeserialize)]
pub fn derive_endian_be_de_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_endian_impl(input, Endian::Big, SerDe::Deserialize)
}

#[proc_macro_derive(MixedEndianDeserialize, attributes(endian))]
pub fn derive_endian_me_de_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_endian_impl(input, Endian::Mixed, SerDe::Deserialize)
}

fn derive_endian_impl(
    input: proc_macro::TokenStream,
    endian: Endian,
    serde: SerDe,
) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: (Big/Little/Mixed)Endian(Serialize/Deserialize)` to every type parameter T.
    let generics = match serde {
        SerDe::Serialize => match endian {
            Endian::Little => add_trait_bounds(input.generics, parse_quote!(LittleEndianSerialize)),
            Endian::Big => add_trait_bounds(input.generics, parse_quote!(BigEndianSerialize)),
            Endian::Mixed => add_trait_bounds(input.generics, parse_quote!(MixedEndianSerialize)),
            Endian::Native => unimplemented!(),
        },
        SerDe::Deserialize => match endian {
            Endian::Little => {
                add_trait_bounds(input.generics, parse_quote!(LittleEndianDeserialize))
            }
            Endian::Big => add_trait_bounds(input.generics, parse_quote!(BigEndianDeserialize)),
            Endian::Mixed => add_trait_bounds(input.generics, parse_quote!(MixedEndianDeserialize)),
            Endian::Native => unimplemented!(),
        },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate an expression to sum up the heap size of each field.
    let body = serde_data_expands(&input.data, endian, serde);

    // The generated impl.
    let expanded = match serde {
        SerDe::Serialize => match endian {
            Endian::Little => quote! {
                impl #impl_generics LittleEndianSerialize for #name #ty_generics #where_clause {
                     fn serialize_as_le_bytes(&self, bytes: &mut [u8]) {
                       #body
                     }
                }
            },
            Endian::Big => quote! {
                impl #impl_generics BigEndianSerialize for #name #ty_generics #where_clause {
                     fn serialize_as_be_bytes(&self, bytes: &mut [u8]) {
                       #body
                     }
                }
            },
            Endian::Mixed => quote! {
                impl #impl_generics MixedEndianSerialize for #name #ty_generics #where_clause {
                     fn serialize_as_me_bytes(&self, bytes: &mut [u8]) {
                       #body
                     }
                }
            },
            Endian::Native => unimplemented!(),
        },
        SerDe::Deserialize => match endian {
            Endian::Little => quote! {
                impl #impl_generics LittleEndianDeserialize for #name #ty_generics #where_clause {
                     fn deserialize_from_le_bytes(bytes: &[u8]) -> Self {
                       Self { #body }
                     }
                }
            },
            Endian::Big => quote! {
                impl #impl_generics BigEndianDeserialize for #name #ty_generics #where_clause {
                     fn deserialize_from_be_bytes(bytes: &[u8]) -> Self {
                       Self { #body }
                     }
                }
            },
            Endian::Mixed => quote! {
                impl #impl_generics MixedEndianDeserialize for #name #ty_generics #where_clause {
                     fn deserialize_from_me_bytes(bytes: &[u8]) -> Self {
                       Self { #body }
                     }
                }
            },
            Endian::Native => unimplemented!(),
        },
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

use syn::{punctuated::Punctuated, token::Comma, Field};

fn serde_fields(fields: &Punctuated<Field, Comma>, endian: Endian, serde: SerDe) -> TokenStream {
    let mut beg_offset = quote! { 0 };
    let mut recurse = vec![];
    for field in fields.iter() {
        let name = &field.ident;
        let ty = &field.ty;
        let struct_size = quote! { <#ty as EndianSize>::BYTES_LEN };
        let end_offset = quote! { #beg_offset + #struct_size };
        let bytes_slice = quote! { bytes[#beg_offset..#end_offset] };
        match serde {
            SerDe::Serialize => match endian {
                Endian::Little => recurse.push(quote_spanned! {field.span()=>
                    debug_assert_eq!(#struct_size, #bytes_slice.len());
                    LittleEndianSerialize::serialize_as_le_bytes(&self.#name, &mut #bytes_slice);
                }),
                Endian::Big => recurse.push(quote_spanned! {field.span()=>
                    debug_assert_eq!(#struct_size, #bytes_slice.len());
                    BigEndianSerialize::serialize_as_be_bytes(&self.#name, &mut #bytes_slice);
                }),
                Endian::Mixed => {
                    let filed_endian = attr::endian_from_attribute(&field.attrs);

                    let r = match filed_endian {
                        Some(Endian::Little) => quote_spanned! {field.span()=>
                            debug_assert_eq!(#struct_size, #bytes_slice.len());
                            LittleEndianSerialize::serialize_as_le_bytes(&self.#name, &mut #bytes_slice);
                        },
                        Some(Endian::Big) => quote_spanned! {field.span()=>
                            debug_assert_eq!(#struct_size, #bytes_slice.len());
                            BigEndianSerialize::serialize_as_be_bytes(&self.#name, &mut #bytes_slice);
                        },
                        Some(Endian::Mixed) | Some(Endian::Native) => unimplemented!(),
                        None => quote_spanned! {field.span()=>
                          debug_assert_eq!(#struct_size, #bytes_slice.len());
                          MixedEndianSerialize::serialize_as_me_bytes(&self.#name, &mut #bytes_slice);
                        },
                    };
                    recurse.push(r)
                }
                Endian::Native => unimplemented!(),
            },
            SerDe::Deserialize => match endian {
                Endian::Little => recurse.push(quote_spanned! {field.span()=>
                    #name: LittleEndianDeserialize::deserialize_from_le_bytes(& #bytes_slice),
                }),
                Endian::Big => recurse.push(quote_spanned! {field.span()=>
                    #name: BigEndianDeserialize::deserialize_from_be_bytes(& #bytes_slice),
                }),
                Endian::Mixed => {
                    let filed_endian = attr::endian_from_attribute(&field.attrs);

                    let r = match filed_endian {
                        Some(Endian::Little) => quote_spanned! {field.span()=>
                            #name: LittleEndianDeserialize::deserialize_from_le_bytes(& #bytes_slice),
                        },
                        Some(Endian::Big) => quote_spanned! {field.span()=>
                            #name: BigEndianDeserialize::deserialize_from_be_bytes(& #bytes_slice),
                        },
                        Some(Endian::Mixed) | Some(Endian::Native) => unimplemented!(),
                        None => quote_spanned! {field.span()=>
                          #name: MixedEndianDeserialize::deserialize_from_me_bytes(& #bytes_slice),
                        },
                    };
                    recurse.push(r)
                }
                Endian::Native => unimplemented!(),
            },
        }
        beg_offset = quote! { #beg_offset + #struct_size }
    }

    quote! {
        #(#recurse)*
    }
}

fn serde_data_expands(data: &Data, endian: Endian, serde: SerDe) -> TokenStream {
    // this also contains `bytes` variable
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => serde_fields(&fields.named, endian, serde),
                Fields::Unnamed(ref fields) => serde_fields(&fields.unnamed, endian, serde),
                Fields::Unit => {
                    // Unit structs cannot own more than 0 bytes of heap memory.
                    quote!(0)
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

// Add a bound `T: trait_bound` to every type parameter T.
fn add_trait_bounds(mut generics: Generics, trait_bound: TypeParamBound) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(trait_bound.clone());
        }
    }
    generics
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
