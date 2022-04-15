use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::Ident;

// TODO: Add documentation to generated shit

/// Implements the builder design pattern as a macro as described here:
/// https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
pub fn impl_builder_macro(ast: &syn::DeriveInput) -> TokenStream {
    if let syn::Data::Struct(struct_data) = &ast.data {

        let name = &ast.ident;
        let builder_name = &quote::format_ident!("{}Builder", name);

        let builder_finder = quote! {
            impl #name {
                pub fn builder() -> #builder_name {
                    #builder_name::default()
                }
            }
        };

        let fields = &struct_data.fields;
        let field_idents: Vec<&Option<Ident>> = struct_data.fields.iter().map(|v| &v.ident).collect();
        let priv_field_idents: Vec<Option<Ident>> = struct_data.fields.iter().map(|v| -> Option<Ident> {
            if let Some(id) = &v.ident {
                Some(quote::format_ident!("set_{}", id))
            } else {
                None
            }
        }).collect();

        let field_tys: Vec<&syn::Type> = fields.iter().map(|v| &v.ty).collect();

        let setters = quote! {
            #(pub fn #field_idents(mut self, #field_idents: #field_tys) -> #builder_name {
                self.#priv_field_idents = Some(#field_idents);
                self
            })*
        };

        let gen = quote! {
            #builder_finder
            
            #[derive(Default)]
            pub struct #builder_name {
                #(#priv_field_idents: Option<#field_tys>,)*
            }

            impl #builder_name {
                pub fn new() -> #builder_name {
                    #builder_name {
                        #(#priv_field_idents: None,)*
                    }
                }

                #setters

                pub fn build(self) -> #name {
                    #name {
                        #(#field_idents: self.#priv_field_idents.unwrap_or_default(),)*
                    }
                }
            }
        };

        gen.into()
    } else {
        let gen = quote_spanned! {
            ast.ident.span()=>
                compile_error!("Ayo, this is for structs only!!");
        };
        gen.into()
    }
}
