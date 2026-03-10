use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DeriveInput, Field, Fields, GenericArgument, Ident, Path, PathArguments,
    Result, Type,
};

struct PoolField<'a> {
    ident: &'a Ident,
    component_ty: &'a Type,
    is_canonical: bool,
    is_required: bool,
}

pub fn derive_entity_pool(item: TokenStream) -> Result<TokenStream> {
    let input = syn::parse2::<DeriveInput>(item)?;

    let ident = input.ident.clone();

    let visibility = input.vis.clone();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let pool_attr = parse_pool_attribute(&input.attrs)?;

    match input.data {
        Data::Enum(enum_data) => Err(syn::Error::new(
            enum_data.enum_token.span,
            "cannot derive `EntityPool` on enums.",
        )),
        Data::Union(union_data) => Err(syn::Error::new(
            union_data.union_token.span,
            "cannot derive `EntityPool` on unions",
        )),
        Data::Struct(struct_data) => {
            let fields = match &struct_data.fields {
                Fields::Named(fields) => &fields.named,
                Fields::Unnamed(fields) => {
                    return Err(syn::Error::new(
                        fields.unnamed.span(),
                        "cannot derive `EntityPool` on tuple structs",
                    ));
                }
                Fields::Unit => {
                    return Err(syn::Error::new(
                        struct_data.struct_token.span,
                        "cannot derive `EntityPool` on unit structs",
                    ));
                }
            };

            let pool_fields = fields
                .iter()
                .map(|f| {
                    let is_canonical = has_attribute(f, "canonical");
                    let is_required = is_canonical || has_attribute(f, "required");
                    let component_ty = extract_inner_type(f)?;
                    Ok(PoolField {
                        ident: f.ident.as_ref().unwrap(),
                        component_ty,
                        is_canonical,
                        is_required,
                    })
                })
                .collect::<syn::Result<Vec<PoolField>>>()?;

            let mut pool_fields_iter = pool_fields.iter();
            let canonical = pool_fields_iter
                .find(|f| f.is_canonical)
                .filter(|_| !pool_fields_iter.any(|f| f.is_canonical))
                .ok_or(syn::Error::new(
                    ident.span(),
                    "EntityPool requires exactly one #[canonical] field",
                ))?;

            let canonical_ident = canonical.ident;

            let entity_kind = pool_attr
                .kind
                .segments
                .last()
                .ok_or(syn::Error::new(
                    pool_attr.span,
                    "expected a non-empty path for `kind`",
                ))?
                .ident
                .clone();

            let bundle_name =
                syn::Ident::new(&format!("{}Bundle", entity_kind), entity_kind.span());

            let required_fields: Vec<&PoolField> =
                pool_fields.iter().filter(|f| f.is_required).collect();
            let all_fields: Vec<&PoolField> = pool_fields.iter().collect();

            let bundle_fields = required_fields.iter().map(|f| {
                let ident = f.ident;
                let component_ty = f.component_ty;
                quote! { pub #ident: #component_ty }
            });

            let spawn_inserts = required_fields.iter().map(|f| {
                let ident = &f.ident;
                quote! {
                    crate::storage::ComponentStore::insert(&mut self.#ident, index, bundle.#ident)?;
                }
            });

            let despawn_removes = all_fields.iter().map(|f| {
                let ident = &f.ident;
                quote! {
                    crate::storage::ComponentStore::remove(&mut self.#ident, index).ok();
                }
            });

            let insert_into_impls = all_fields.iter().map(|f| {
                let field_ident = f.ident;
                let component_ty = f.component_ty;
                quote! {
                    impl #impl_generics crate::traits::InsertInto<#ident #ty_generics> for #component_ty {
                        fn insert_into(
                            self,
                            pool: &mut #ident #ty_generics,
                            index: u8,
                        ) -> ::core::result::Result<(), crate::errors::ComponentStorageError> {
                            crate::storage::ComponentStore::insert(&mut pool.#field_ident, index, self)
                        }
                    }
                }
            });

            let remove_from_impls = all_fields.iter().map(|f| {
                let field_ident = f.ident;
                let component_ty = f.component_ty;
                quote! {
                    impl #impl_generics crate::traits::RemoveFrom<#ident #ty_generics> for #component_ty {
                        fn remove_from(
                            pool: &mut #ident #ty_generics,
                            index: u8,
                        ) -> ::core::result::Result<(), crate::errors::ComponentStorageError> {
                            crate::storage::ComponentStore::remove(&mut pool.#field_ident, index)
                        }
                    }
                }
            });

            Ok(quote! {

                #[derive(Debug)]
                #visibility struct #bundle_name {
                    #(#bundle_fields,)*
                }


                impl #impl_generics #ident #ty_generics #where_clause {

                    // Find the first slot not occupied by the canonical component
                    fn first_free(&self) -> ::core::option::Option<u8> {
                        // we are assuming that the pool has the const generic N, which isn't guaranteed but is probably good enough for this derive macro
                        (0..N as u8).find(|&i| {
                            !crate::storage::ComponentStore::contains(&self.#canonical_ident, i)
                        })
                    }

                    fn spawn(
                        &mut self,
                        index: u8,
                        bundle: #bundle_name
                    ) -> ::core::result::Result<
                        crate::entity::EntityRef<'_, #ident #ty_generics>,
                        crate::errors::ComponentStorageError
                    > {

                        let index = self
                        .first_free()
                        .ok_or(crate::errors::ComponentStorageError::PoolFull)?;

                        #(#spawn_inserts)*
                        Ok(crate::entity::EntityRef {
                            pool: self,
                            index,
                        })
                    }

                    fn despawn(&mut self, index: u8) {
                        #(#despawn_removes)*
                    }
                }

                #(#insert_into_impls)*
                #(#remove_from_impls)*
            })
        }
    }
}

fn has_attribute(field: &Field, attr_name: &str) -> bool {
    field
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident(attr_name))
}

fn extract_inner_type(field: &Field) -> Result<&Type> {
    let Type::Path(tp) = &field.ty else {
        return Err(syn::Error::new(
            field.ty.span(),
            "EntityPool fields must be a SparseSet<T, N> or MarkerSet<T, N>",
        ));
    };
    let segment = tp.path.segments.last().ok_or(syn::Error::new(
        field.ty.span(),
        "expected a type path with at least one segment",
    ))?;

    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return Err(syn::Error::new(
            segment.ident.span(),
            "missing generic arguments, expected `SparseSet<T, N>` or `MarkerSet<T, N>`",
        ));
    };

    let first_arg = args.args.first().ok_or(syn::Error::new(
        segment.ident.span(),
        "missing type argument, expected `SparseSet<T, N>` or `MarkerSet<T, N>`",
    ))?;

    let GenericArgument::Type(inner_ty) = first_arg else {
        return Err(syn::Error::new(
            segment.ident.span(),
            "first generic argument must be a type, e.g. `SparseSet<Health, N>` where `Health` is \
             a component",
        ));
    };

    Ok(inner_ty)
}

struct PacketAttr {
    span: Span,
    kind: Path,
}

fn parse_pool_attribute(attributes: &[Attribute]) -> Result<PacketAttr> {
    let Some(attribute) = attributes
        .iter()
        .find(|&attribute| attribute.path().is_ident("pool"))
    else {
        return Err(syn::Error::new(
            Span::call_site(),
            "missing required `pool` argument for `EntityPool` derive macro",
        ));
    };

    let mut kind = None;

    attribute.parse_nested_meta(|meta| {
        if meta.path.is_ident("kind") {
            kind = Some(meta.value()?.parse::<Path>()?);
            Ok(())
        } else {
            Err(meta.error("unsupported pool argument"))
        }
    })?;

    let Some(kind) = kind else {
        return Err(syn::Error::new(
            attribute.span(),
            "missing required `kind = ...` argument from `pool` attribute for `EntityPool` derive \
             macro",
        ));
    };

    Ok(PacketAttr {
        span: attribute.span(),
        kind,
    })
}
