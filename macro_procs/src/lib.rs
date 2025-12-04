extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ScreenEngine)]
pub fn screen_engine_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl ScreenEngine for #name{
            type Pixels<'a> = &'a PixelsType;
            fn pixels(&self) -> Self::Pixels<'_> {
                &self.pixels
            }
        }
    };

    TokenStream::from(expanded)
}
