//! A set of custom macros for use in character builder
//!
//! Provides some sanity for myself when I need it most.
#![doc(html_playground_url = "https://play.rust-lang.org/")]

#![warn(missing_docs)]

mod builder;

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};

/// Setup a Builder for a struct
#[proc_macro_derive(Builder)]
pub fn builder_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    builder::impl_builder_macro(&ast)
}

/// Setup Serde Deserialization for an Asset
#[proc_macro_derive(DeserializeAsset)]
pub fn deserializeasset_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_deserializeasset_macro(&ast)
}
fn impl_deserializeasset_macro(ast: &syn::DeriveInput) -> TokenStream {
    if let syn::Data::Enum(enum_data) = &ast.data {
        let name = &ast.ident;
        let unsafe_name = &quote::format_ident!("Unsafe{}", name);
        let variants = &enum_data.variants;
        let variant_idents = enum_data.variants.iter().map(|v| &v.ident);
        let _variant_names = enum_data.variants.iter().map(|v| v.ident.to_string());

        let unsafe_enum = quote! {
            #[derive(Deserialize)]
            enum #unsafe_name {
                #variants
            }
        };

        let gen = quote! {
            impl<'de> ::serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    #unsafe_enum

                    impl TryFrom<#unsafe_name> for #name {
                        type Error = String;

                        fn try_from(ue: #unsafe_name) -> Result<Self, Self::Error> {

                            match ue {
                                #(#unsafe_name::#variant_idents {metadata, spec} => {
                                        Ok(Self::#variant_idents{ue})
                                },)*
                                _ => Err(String::from("What?")),
                            }
                        }
                    }


                    #unsafe_name::deserialize(deserializer)?.try_into().map_err(::serde::de::Error::custom)
                }
            }

        };

        gen.into()
    } else {
        let gen = quote_spanned! {
            ast.ident.span()=>
                compile_error!("Ayo, this is for an enum only!!");
        };
        gen.into()
    }
}

/// Macro to setup the SelectEnum
#[proc_macro_derive(SelectEnum)]
pub fn selectenum_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_selectenum_macro(&ast)
}

fn impl_selectenum_macro(ast: &syn::DeriveInput) -> TokenStream {
    if let syn::Data::Enum(enum_data) = &ast.data {
        let variants = enum_data.variants.iter().map(|v| &v.ident);

        let variant_docs = enum_data.variants.iter().map(|v| &v.ident).map(|id| format!("A Selector for the {} Asset", id));

        let gen = quote! {
            /// SelectVariant
            #[derive(Debug, PartialEq, Serialize)]
            pub struct SelectVariant {
                /// Pretty Name to be displayed
                pub name: String,
                /// The number of ids that can be selected
                pub number: usize,
                /// The ids that may be selected
                pub id: Vec<String>,
            }

            impl<'de> ::serde::Deserialize<'de> for SelectVariant {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {

                    #[derive(Deserialize)]
                    struct UnsafeSelectVariant {
                        name: String,
                        number: usize,
                        id: Vec<String>,
                    }

                    impl TryFrom<UnsafeSelectVariant> for SelectVariant {
                        type Error = usize;

                        fn try_from(us: UnsafeSelectVariant) -> Result<Self, Self::Error> {
                            if (us.number < us.id.len()) {
                                Ok(Self {
                                    name: us.name,
                                    number: us.number,
                                    id: us.id,
                                })
                            } else {
                                Err(us.number)
                            }
                        }
                    }

                    let fixify = |e: usize| ::serde::de::Error::invalid_value(::serde::de::Unexpected::Other(&format!("integer `{}`", e)), &"an integer less than the total number of id(s)");

                    UnsafeSelectVariant::deserialize(deserializer)?.try_into().map_err(fixify)
                }
            }

            /// Select
            #[derive(Debug, PartialEq, Serialize, Deserialize)]
            #[serde(rename_all = "lowercase", tag = "type")]
            pub enum Select {
                #(#[doc = #variant_docs] #variants (SelectVariant),)*
            }
        };

        gen.into()
    } else {
        let gen = quote_spanned! {
            ast.ident.span()=>
                compile_error!("Ayo, this is for an enum only!!");
        };
        gen.into()
    }
}

/// Add [`std::str::FromStr`] trait to convert strings into enum variants
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate my_macros;
/// ##[derive(EnumString)]
/// enum E {
///     Foo,
///     Bar(String),
/// }
/// ```
///
/// ```
/// # #[macro_use] extern crate my_macros;
/// # #[derive(EnumString)]
/// # enum E {
/// #    Foo,
/// #    Bar(String),
/// # }
/// # fn main() {
/// E::from_str("foo").unwrap();
/// # }
/// ```
/// Will panic if you try to convert a string to a variant that does not exist
/// ```should_panic
/// # #[macro_use] extern crate my_macros;
/// # #[derive(EnumString)]
/// # enum E {
/// #    Foo,
/// #    Bar(String),
/// # }
/// # fn main() {
/// E::from_str("undefined").unwrap();
/// # }
/// ```
///
/// This macro will only work with enums and thus will fail if you try using
/// it with structs.
/// ```compile_fail
/// # #[macro_use] extern crate my_macros;
/// ##[derive(EnumString)]
/// struct E {
///     foo: String,
/// }
/// ```
#[proc_macro_derive(EnumString)]
pub fn enumstring_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_enumstring_macro(&ast)
}

fn impl_enumstring_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    if let syn::Data::Enum(enum_data) = &ast.data {
        let variants = enum_data.variants.iter().map(|v| &v.ident);

        let variant_lowers = enum_data
            .variants
            .iter()
            .map(|v| v.ident.to_string().to_lowercase());

        let params = enum_data.variants.iter().map(|v| match &v.fields {
            syn::Fields::Unit => quote! {},
            syn::Fields::Unnamed(fields) => {
                let defaults =
                    ::std::iter::repeat(quote!(Default::default())).take(fields.unnamed.len());
                quote! { (#(#defaults),*) }
            }
            syn::Fields::Named(fields) => {
                let fields = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                quote! { {#(#fields: Default::default()),*} }
            }
        });

        let gen = quote! {

            use ::std::str::FromStr;
            impl FromStr for #name {
                type Err = ();

                fn from_str(input: &str) -> Result<#name, Self::Err> {
                    match input {
                        #(#variant_lowers => Ok(#name::#variants #params),)*
                        _ => Err(()),
                    }
                }
            }
        };
        gen.into()
    } else {
        let gen = quote_spanned! {
            ast.ident.span()=>
            compile_error!("Ayo, this is for an enum only!!");
        };
        gen.into()
    }
}
