use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    spanned::Spanned,
    token::{Enum, Union},
    Data, DataEnum, DataUnion, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed, Index,
};

#[proc_macro_derive(StorageProvider, attributes(storage_field))]
pub fn storage_provider(tokens: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(tokens as DeriveInput);

    let (all_fields, fields_w_attr) = match &derive_input.data {
        Data::Struct(structure) => match &structure.fields {
            Fields::Named(FieldsNamed { named: fields, .. })
            | Fields::Unnamed(FieldsUnnamed {
                unnamed: fields, ..
            }) => (
                fields,
                fields
                    .iter()
                    .enumerate()
                    .filter_map(|(field_index, field)| {
                        if field
                            .attrs
                            .iter()
                            .any(|attr| attr.path.is_ident("storage_field"))
                        {
                            Some((
                                if let Some(ident) = &field.ident {
                                    ident.into_token_stream()
                                } else {
                                    Index::from(field_index).into_token_stream()
                                },
                                field.ty.to_token_stream(),
                            ))
                        } else {
                            None
                        }
                    }),
            ),
            Fields::Unit => return Error::new(
                derive_input.span(),
                "`StorageProvider` deriving does nothing for unit (empty) structs, add some fields",
            )
            .into_compile_error()
            .into(),
        },
        Data::Enum(DataEnum {
            enum_token: Enum { span },
            ..
        })
        | Data::Union(DataUnion {
            union_token: Union { span },
            ..
        }) => {
            return Error::new(
                *span,
                "`StorageProvider` deriving is supported only with structs",
            )
            .into_compile_error()
            .into()
        }
    };

    let struct_ident = derive_input.ident;

    let implementations = fields_w_attr.map(|(field_ident, field_type)| {
        quote::quote! {
            impl StorageProvider<#field_type> for #struct_ident {
                fn storage(&self) -> &#field_type {
                    &self.#field_ident
                }

                fn storage_mut(&mut self) -> &mut #field_type {
                    &mut self.#field_ident
                }
            }
        }
    });

    let token_stream = quote::quote! {
        #(#implementations)*
    };

    if token_stream.is_empty() {
        Error::new(all_fields.span(), "`StorageProvider` deriving does nothing if no field has the `#[storage_field]` attribute").into_compile_error().into()
    } else {
        token_stream.into()
    }
}
