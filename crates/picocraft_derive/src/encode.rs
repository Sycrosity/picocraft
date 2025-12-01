use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Expr, Fields, LitInt, Result};

pub fn derive_encode(item: TokenStream) -> Result<TokenStream> {
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
                     Encode on enums",
                ));
            };

            enum_data
                .variants
                .iter()
                .find(|variant| !variant.fields.is_empty())
                .map_or(Ok(()), |variant| {
                    Err(syn::Error::new(
                        variant.fields.span(),
                        "cannot derive `Encode` on enums with fields",
                    ))
                })?;

            enum_data
                .variants
                .iter()
                .find(|&variant| variant.discriminant.is_none())
                .map_or(Ok(()), |variant| {
                    Err(syn::Error::new(
                        variant.span(),
                        "cannot derive `Encode` on enums without explicit discriminants",
                    ))
                })?;

            let encode_fields: TokenStream = enum_data
                .variants
                .iter()
                .map(|variant| {
                    let (_, expr) = variant.discriminant.clone().unwrap();

                    let ident = variant.ident.clone();

                    quote! {
                        Self::#ident => #enum_type::from(#expr).encode(buffer).await?,
                    }
                })
                .collect();

            Ok(quote! {
                impl #impl_generics ::picocraft_core::packet::Encode for #ident #ty_generics #where_clause {
                    async fn encode<W>(&self, mut buffer: W) -> ::core::result::Result<(), ::picocraft_core::packet::EncodeError<W::Error>>
                    where W: ::embedded_io_async::Write {

                        use ::picocraft_core::packet::Encode;

                        match self {
                            #encode_fields
                        }

                        Ok(())
                    }
                }
            })
        }
        Data::Union(union_data) => Err(syn::Error::new(
            union_data.union_token.span,
            "cannot derive `Encode` on unions",
        )),
        Data::Struct(struct_data) => {
            let encode_fields = match &struct_data.fields {
                Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|field| {
                        let name = field.ident.as_ref().unwrap();
                        quote! {
                            self.#name.encode(buffer).await?;
                        }
                    })
                    .collect(),
                Fields::Unnamed(fields) => (0..fields.unnamed.len())
                    .map(|i| {
                        let lit = LitInt::new(&i.to_string(), Span::call_site());
                        quote! {
                            self.#lit.encode(buffer).await?;
                        }
                    })
                    .collect(),
                Fields::Unit => TokenStream::new(),
            };
            Ok(quote! {
                impl #impl_generics ::picocraft_core::packet::Encode for #ident #ty_generics #where_clause {
                    async fn encode<W>(&self, mut buffer: W) -> ::core::result::Result<(), ::picocraft_core::packet::EncodeError<W::Error>>
                    where W: ::embedded_io_async::Write {

                        use ::picocraft_core::packet::Encode;

                        #encode_fields

                        Ok(())
                    }
                }
            })
        }
    }
}

struct EncodeAttr {
    span: Span,
    enum_type: Option<Expr>,
}

fn parse_protocol_attribute(attributes: &[Attribute]) -> Result<Option<EncodeAttr>> {
    let Some(attribute) = attributes
        .iter()
        .find(|&attribute| attribute.path().is_ident("protocol"))
    else {
        return Ok(None);
    };

    let mut result = EncodeAttr {
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
