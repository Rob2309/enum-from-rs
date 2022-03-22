use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput, Data, spanned::Spanned, Fields};

/// 
#[proc_macro_derive(From, attributes(from))]
pub fn derive_from(target: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let decl = parse_macro_input!(target as DeriveInput);

    let enum_data = if let Data::Enum(e) = decl.data { e } else { return error("#[derive(From)] can only be used on enums", decl.span()); };
    let enum_name = decl.ident;

    let mut derives = proc_macro2::TokenStream::new();

    for variant in enum_data.variants {
        let variant_name = variant.ident;

        let fields = match variant.fields {
            Fields::Named(fields) => {
                fields.named
            },
            Fields::Unnamed(fields) => {
                fields.unnamed
            },
            Fields::Unit => continue,
        };

        let field = if let Some(field) = fields.first() { field } else { continue; };
        if !field.attrs.iter().any(|attr| attr.path.is_ident("from")) {
            continue;
        }

        if fields.len() != 1 {
            return error("#[from] can only be used on variants with one element", fields.span());
        }

        let field_type = &field.ty;

        if let Some(field_name) = &field.ident {
            derives.extend(quote! {
                impl ::core::convert::From<#field_type> for #enum_name {
                    fn from(from: #field_type) -> Self {
                        Self::#variant_name {
                            #field_name: from,
                        }
                    }
                }
            });
        } else {
            derives.extend(quote! {
                impl ::core::convert::From<#field_type> for #enum_name {
                    fn from(from: #field_type) -> Self {
                        Self::#variant_name (
                            from
                        )
                    }
                }
            });
        }
    }

    derives.into()
}

fn error(msg: &str, span: Span) -> proc_macro::TokenStream {
    quote_spanned! {span=>
        compile_error!(#msg);
    }.into()
}
