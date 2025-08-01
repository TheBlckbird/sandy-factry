use foreground_objects::impl_foreground_objects_macro;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod foreground_objects;

#[proc_macro_derive(ForegroundObjects, attributes(variant))]
pub fn foreground_objects_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_foreground_objects_macro(&ast)
}
