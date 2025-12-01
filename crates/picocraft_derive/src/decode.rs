use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Expr, Fields, Result};

pub fn derive_decode(item: TokenStream) -> Result<TokenStream> {
    let input = syn::parse2::<DeriveInput>(item)?;

    let ident = input.ident.clone();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Some(encode_attr) = parse_protocol_attribute(&input.attrs)? else {
        return Err(syn::Error::new(
            input.span(),
            "`packet(...)` attribute is required.",
        ));
    };

    match input.data {
        Data::Enum(enum_data) => {
            let Some(enum_type) = encode_attr.enum_type else {
                return Err(syn::Error::new(
                    encode_attr.span,
                    "`enum_type = ...` value from packet attribute is required for deriving \
                     Decode on enums",
                ));
            };

            enum_data
                .variants
                .iter()
                .find(|variant| !variant.fields.is_empty())
                .map_or(Ok(()), |variant| {
                    Err(syn::Error::new(
                        variant.fields.span(),
                        "cannot derive `Decode` on enums with fields",
                    ))
                })?;

            enum_data
                .variants
                .iter()
                .find(|&variant| variant.discriminant.is_none())
                .map_or(Ok(()), |variant| {
                    Err(syn::Error::new(
                        variant.span(),
                        "cannot derive `Decode` on enums without explicit discriminants",
                    ))
                })?;

            let decode_fields: TokenStream = enum_data
                .variants
                .iter()
                .map(|variant| {
                    let (_, expr) = variant.discriminant.clone().unwrap();

                    let ident = variant.ident.clone();

                    quote! {
                        #expr => Ok(Self::#ident),
                    }
                })
                .collect();

            Ok(quote! {

                impl #impl_generics ::picocraft_core::packet::Decode for #ident #ty_generics #where_clause {
                    async fn decode<R>(mut buffer: R) -> ::core::result::Result<Self,::picocraft_core::packet::DecodeError<R::Error>>
                    where R: ::embedded_io_async::Read {

                        use ::picocraft_core::packet::Decode;

                        let int = #enum_type::decode(&mut buffer).await?;

                        match i32::from(int) {

                            #decode_fields
                            _ => Err(::picocraft_core::packet::DecodeError::InvalidEnumValue)

                        }
                    }
                }
            })
        }
        Data::Union(union_data) => Err(syn::Error::new(
            union_data.union_token.span,
            "cannot derive `Decode` on unions",
        )),
        Data::Struct(struct_data) => {
            let decode_fields = match &struct_data.fields {
                Fields::Named(fields) => {
                    let init = fields.named.iter().map(|field| {
                        let name = field.ident.clone().unwrap();
                        let ty = field.ty.clone();
                        quote! {
                            #name: <#ty as ::picocraft_core::prelude::Decode>::decode(buffer).await?,
                        }
                    });

                    quote! {
                        Self {
                            #(#init)*
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let init = (0..fields.unnamed.len())
                        .map(|_| {
                            quote! {
                                ::picocraft_core::prelude::Decode::decode(buffer).await?,
                            }
                        })
                        .collect::<TokenStream>();

                    quote! {
                        Self(#init)
                    }
                }
                Fields::Unit => quote!(Self),
            };

            Ok(quote! {

                impl #impl_generics ::picocraft_core::packet::Decode for #ident #ty_generics #where_clause {
                    async fn decode<R>(mut buffer: R) -> ::core::result::Result<Self,::picocraft_core::packet::DecodeError<R::Error>>
                    where R: ::embedded_io_async::Read {

                        use ::picocraft_core::packet::Decode;

                        Ok(#decode_fields)
                    }
                }
            })
        }
    }
}

struct DecodeAttr {
    span: Span,
    enum_type: Option<Expr>,
}

fn parse_protocol_attribute(attributes: &[Attribute]) -> Result<Option<DecodeAttr>> {
    let Some(attribute) = attributes
        .iter()
        .find(|&attribute| attribute.path().is_ident("protocol"))
    else {
        return Ok(None);
    };

    let mut result = DecodeAttr {
        span: attribute.span(),
        enum_type: None,
    };

    attribute.parse_nested_meta(|meta| {
        if meta.path.is_ident("enum_type") {
            result.enum_type = Some(meta.value()?.parse::<syn::Expr>()?);
            Ok(())
        } else {
            Err(meta.error("unsupported procotol argument"))
        }
    })?;

    Ok(Some(result))
}
