#![no_std]

/// This `macro_rule` generates a procedural derive macro for storage trait.
///
/// The first argument is the name of the procedural function.
/// The second argument is the name of the trait for which derive will be generated.
/// The third argument is the name of the marker for the derive macro. This marker specifies
/// for derive macro which field will be returned by the implementation for the storage trait.
macro_rules! declare_derive_storage_trait {
    ($derive_name:ident,$trait_name:ident,$trait_field_specifier:ident) => {
        #[proc_macro_derive($trait_name, attributes($trait_field_specifier))]
        pub fn $derive_name(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let derive = syn::parse_macro_input!(_item as syn::DeriveInput);
            const FIELD_SETTER: &'static str = stringify!($trait_field_specifier);

            let struct_ident = derive.ident;

            let field_ident;
            let field_ty;
            if let syn::Data::Struct(data) = &derive.data {
                if let syn::Fields::Named(named_fields) = &data.fields {
                    let field = named_fields.named.iter().find(|f| {
                        f.attrs
                            .iter()
                            .find(|a| a.path().is_ident(FIELD_SETTER))
                            .is_some()
                    });

                    if let Some(field) = field {
                        field_ident = field.ident.clone();
                        field_ty = field.ty.clone();
                    } else {
                        return quote::quote! {
                            panic!();
                        }
                        .into();
                    }
                } else {
                    return quote::quote! {
                        panic!("not supported field");
                    }
                    .into();
                }
            } else {
                return quote::quote! {
                    panic!("only supports struct");
                }
                .into();
            }

            let code = quote::quote! {
                impl $trait_name for #struct_ident {
                    fn get(&self) -> & #field_ty {
                        &self.#field_ident
                    }

                    fn get_mut(&mut self) -> &mut #field_ty {
                        &mut self.#field_ident
                    }
                }
            };
            code.into()
        }
    };
}

macro_rules! declare_impl_trait {
    ($derive_name:ident, $trait_core_name:ident) => {
        #[proc_macro_derive($trait_core_name)]
        pub fn $derive_name(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let derive = syn::parse_macro_input!(_item as syn::DeriveInput);
            let struct_ident = derive.ident;
            let code = quote::quote! {
                impl $trait_core_name for #struct_ident {}
            };
            code.into()
        }
    };
}

// NFT
declare_derive_storage_trait!(derive_nft_state, NFTStateKeeper, NFTStateField);
declare_impl_trait!(derive_nft_core, NFTCore);
declare_impl_trait!(derive_nft_metastate, NFTMetaState);

// MultiToken
declare_derive_storage_trait!(derive_mtk_state, StateKeeper, MTKStateKeeper);
declare_impl_trait!(derive_mtk_token_state, MTKTokenState);
declare_impl_trait!(derive_mtk_core, MTKCore);

// FT
declare_derive_storage_trait!(derive_ft_state, FTStateKeeper, FTStateField);
declare_impl_trait!(derive_ft_core, FTCore);
declare_impl_trait!(derive_ft_metastate, FTMetaState);
