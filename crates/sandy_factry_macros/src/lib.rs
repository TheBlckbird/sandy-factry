use foreground_objects::impl_foreground_objects_macro;
use machine_variants::impl_machine_variants_macro;
use proc_macro::TokenStream;

mod foreground_objects;
mod machine_variants;

#[proc_macro_derive(MachineVariants, attributes(machine_type, machine, variant, render))]
pub fn machine_variants_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Invalid Rust Code");
    impl_machine_variants_macro(&ast)
}

#[proc_macro_derive(ForegroundObjects, attributes(variant))]
pub fn foreground_objects_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Invalid Rust Code");
    impl_foreground_objects_macro(&ast)
}
