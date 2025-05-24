use foreground_objects::impl_foreground_objects_macro;
use proc_macro::TokenStream;

mod foreground_objects;

#[proc_macro_derive(ForegroundObjects, attributes(variant))]
pub fn foreground_objects_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Invalid Rust Code");
    impl_foreground_objects_macro(&ast)
}
