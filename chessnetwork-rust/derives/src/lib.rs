use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data};

#[proc_macro_derive(EnumName)]
pub fn enum_name_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = match input.data {
        Data::Enum(e) => e.variants.into_iter().map(|v| v.ident).collect::<Vec<_>>(),
        _ => {
            return syn::Error::new_spanned(name, "EnumName can only be derived on enums")
                .to_compile_error()
                .into();
        }
    };

    let arms = variants.iter().map(|v| {
        let ident = v;
        let s = ident.to_string();
        quote! { #name::#ident => #s }
    });

    let expanded = quote! {
        impl #name {
            pub fn name(&self) -> &'static str {
                match self {
                    #(#arms),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(FromI32)]
pub fn from_i32_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = match input.data {
        Data::Enum(e) => e.variants.into_iter().map(|v| v.ident).collect::<Vec<_>>(),
        _ => {
            return syn::Error::new_spanned(name, "FromI32 can only be derived on enums")
                .to_compile_error()
                .into();
        }
    };

    let arms = variants.iter().map(|v| {
        quote! { x if x == #name::#v as i32 => Ok(#name::#v) }
    });

    let expanded = quote! {
        impl std::convert::TryFrom<i32> for #name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    #(#arms),*,
                    _ => Err(()),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
