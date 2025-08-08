use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(FromTxt)]
pub fn from_txt_macro(input: TokenStream) -> TokenStream {
        let input = parse_macro_input!(input as DeriveInput);
        todo!();
}
