use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Expr, Fields, LitInt, Result, parse_quote};

pub fn derive_packet(item: TokenStream) -> Result<TokenStream> {
    let input = syn::parse2::<DeriveInput>(item)?;

    let ident = input.ident.clone();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let Some(packet_attr) = parse_packet_attribute(&input.attrs)? else {
        return Err(syn::Error::new(
            input.span(),
            "`packet(...)` attribute is required.",
        ));
    };

    let Some(id) = packet_attr.id else {
        return Err(syn::Error::new(
            packet_attr.span,
            "`id = ...` value from packet attribute is required",
        ));
    };

    let state = packet_attr
        .state
        .unwrap_or(parse_quote!(::picocraft_core::state::State::Play));

    match input.data {
        Data::Enum(enum_data) => Err(syn::Error::new(
            enum_data.enum_token.span,
            "cannot derive `Packet` on enums.",
        )),
        Data::Union(union_data) => Err(syn::Error::new(
            union_data.union_token.span,
            "cannot derive `Packet` on unions",
        )),
        Data::Struct(struct_data) => {
            let decode_fields = match &struct_data.fields {
                Fields::Named(fields) => {
                    let init = fields.named.iter().map(|field| {
                        let ty = field.ty.clone();
                        let name = field.ident.as_ref().unwrap();

                        quote! {
                            #name: {::log::trace!("Processing {} field of {}", stringify!(#name), stringify!(#ident)); <#ty as ::picocraft_core::packet::Decode>::decode(&mut buffer).await?},
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
                                ::picocraft_core::prelude::Decode::decode(&mut buffer).await?,
                            }
                        })
                        .collect::<TokenStream>();

                    quote! {
                        Self(#init)
                    }
                }
                Fields::Unit => quote!(Self),
            };

            let encode_fields = match &struct_data.fields {
                Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|f| {
                        let name = &f.ident.as_ref().unwrap();
                        quote! {
                            self.#name.encode(&mut buffer).await?;
                        }
                    })
                    .collect(),
                Fields::Unnamed(fields) => (0..fields.unnamed.len())
                    .map(|i| {
                        let lit = LitInt::new(&i.to_string(), Span::call_site());
                        quote! {
                            self.#lit.encode(&mut buffer).await?;
                        }
                    })
                    .collect(),
                Fields::Unit => TokenStream::new(),
            };
            Ok(quote! {
                impl #impl_generics ::picocraft_core::packet::Encode for #ident #ty_generics #where_clause {
                    async fn encode<W>(&self, mut buffer: W) -> ::core::result::Result<(), ::picocraft_core::packet::EncodeError<W::Error>>
                    where W: ::embedded_io_async::Write {

                        use ::picocraft_core::prelude::*;

                        ::picocraft_core::types::VarInt(#id).encode(&mut buffer).await.inspect_err(|e| ::log::error!("{e:?}"))?;
                        #encode_fields

                        Ok(())
                    }
                }

                impl #impl_generics ::picocraft_core::packet::Decode for #ident #ty_generics #where_clause {
                    async fn decode<R>(mut buffer: R) -> ::core::result::Result<Self,::picocraft_core::packet::DecodeError<R::Error>>
                    where R: ::embedded_io_async::Read {

                        use ::picocraft_core::prelude::*;

                        ::core::result::Result::Ok(#decode_fields)
                    }
                }

                impl #impl_generics ::picocraft_core::packet::Packet for #ident #ty_generics #where_clause {

                    const ID: VarInt = ::picocraft_core::types::VarInt(#id);
                    const STATE: ::picocraft_core::state::State = #state;

                    fn id(&self) -> ::picocraft_core::types::VarInt {

                        Self::ID
                    }

                    fn state(&self) -> ::picocraft_core::state::State {

                        Self::STATE
                    }
                }

                // impl #impl_generics ::core::convert::From<::picocraft_core::packet::RawPacket<'_>> for #ident #ty_generics #where_clause {
                //     fn from(value: ::picocraft_core::packet::RawPacket<'_>) {

                //         <#ident as ::picocraft_core::packet::Decode>::decode(&mut value.data)
                //             .await
                //             .expect("failed to decode packet from RawPacket")

                //     }
                // }
            })
        }
    }
}

struct PacketAttr {
    span: Span,
    //TODO shouldn't be an option, this is required
    id: Option<i32>,
    /// Defaults to [`State::Play`](picocraft_core::state::State::Play).
    state: Option<Expr>,
}

fn parse_packet_attribute(attributes: &[Attribute]) -> Result<Option<PacketAttr>> {
    let Some(attribute) = attributes
        .iter()
        .find(|&attribute| attribute.path().is_ident("packet"))
    else {
        return Ok(None);
    };

    let mut result = PacketAttr {
        span: attribute.span(),
        id: None,
        state: None,
    };

    attribute.parse_nested_meta(|meta| {
        if meta.path.is_ident("id") {
            result.id = Some(meta.value()?.parse::<LitInt>()?.base10_parse::<i32>()?);
            Ok(())
        } else if meta.path.is_ident("state") {
            result.state = Some(meta.value()?.parse::<Expr>()?);
            Ok(())
        } else {
            Err(meta.error("unrecognized packet argument"))
        }
    })?;

    Ok(Some(result))
}
