use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Ident, LitBool, LitInt, spanned::Spanned};

#[derive(Debug)]
enum Side {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Variant {
    name: Ident,
    inputs: Option<Vec<Side>>,
    outputs: Option<Vec<Side>>,
    texture: LitInt,
    should_render: bool,
    machine: Expr,
}

impl Variant {
    fn new(
        name: Ident,
        inputs: Option<Vec<Side>>,
        outputs: Option<Vec<Side>>,
        texture: LitInt,
        should_render: bool,
        machine: Expr,
    ) -> Self {
        Self {
            name,
            inputs,
            outputs,
            texture,
            should_render,
            machine,
        }
    }
}

pub fn impl_machine_variants_macro(ast: &DeriveInput) -> TokenStream {
    println!("{ast:#?}");

    let name = &ast.ident;
    let name_str = name.to_string();
    let attrs = &ast.attrs;

    let mut machine_type = None;
    let mut should_render = false;
    let mut machine: Option<Expr> = None;

    let enum_variants = if let Data::Enum(data_enum) = &ast.data {
        &data_enum.variants
    } else {
        return syn::Error::new_spanned(ast, "MachineVariants can only be derived for enums")
            .to_compile_error()
            .into();
    };

    for attr in attrs {
        if attr.path().is_ident("machine_type") {
            if let Ok(Expr::Path(path)) = attr.parse_args() {
                machine_type = Some(path);
            }
        } else if attr.path().is_ident("render") {
            should_render = true;
        } else if attr.path().is_ident("machine") {
            match attr.parse_args::<Expr>() {
                Ok(machine_expr) => machine = Some(machine_expr),
                Err(err) => return err.to_compile_error().into(),
            };
        } else if attr.path().is_ident("variant") {
            return syn::Error::new(attr.span(), "#[variant] can only be used on enum variants")
                .to_compile_error()
                .into();
        }
    }

    let Some(machine_type) = machine_type else {
        return syn::Error::new_spanned(ast, "Missing #[machine_type(...)] attribute")
            .to_compile_error()
            .into();
    };

    let mut variants = Vec::new();

    for variant in enum_variants {
        let name = &variant.ident;
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        let mut texture_index = None;
        let mut should_render = should_render;
        let mut machine = machine.clone();

        for attr in &variant.attrs {
            if attr.path().is_ident("machine_type")
                || attr.path().is_ident("machine")
                || attr.path().is_ident("render")
            {
                return syn::Error::new(
                    attr.span(),
                    format!(
                        "#[{}{}] can only be used on the enum itself",
                        attr.path().get_ident().expect("This ident should exist"),
                        if attr.path().is_ident("render") {
                            ""
                        } else {
                            "(...)"
                        }
                    ),
                )
                .to_compile_error()
                .into();
            }
        }

        let mut variant_attrs = variant
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("variant"));

        if variant_attrs.clone().count() > 1 {
            return syn::Error::new(
                variant.span(),
                "Only one #[variant(...)] attribute is allowed per enum variant",
            )
            .to_compile_error()
            .into();
        }

        match variant_attrs.next() {
            Some(attr) => {
                let parsing_result = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("texture") {
                        let value = meta.value()?;
                        let texure_value: LitInt = value.parse()?;
                        texture_index = Some(texure_value);

                        Ok(())
                    } else if meta.path.is_ident("render") {
                        let value = meta.value()?;
                        let should_render_value: LitBool = value.parse()?;
                        should_render = should_render_value.value();

                        Ok(())
                    } else if meta.path.is_ident("inputs") {
                        meta.parse_nested_meta(|nested_meta| {
                            if nested_meta.path.is_ident("North") {
                                inputs.push(Side::North);
                                Ok(())
                            } else if nested_meta.path.is_ident("East") {
                                inputs.push(Side::East);
                                Ok(())
                            } else if nested_meta.path.is_ident("South") {
                                inputs.push(Side::South);
                                Ok(())
                            } else if nested_meta.path.is_ident("West") {
                                inputs.push(Side::West);
                                Ok(())
                            } else {
                                Err(meta.error("unsupported side"))
                            }
                        })?;

                        Ok(())
                    } else if meta.path.is_ident("outputs") {
                        meta.parse_nested_meta(|nested_meta| {
                            if nested_meta.path.is_ident("North") {
                                outputs.push(Side::North);
                                Ok(())
                            } else if nested_meta.path.is_ident("East") {
                                outputs.push(Side::East);
                                Ok(())
                            } else if nested_meta.path.is_ident("South") {
                                outputs.push(Side::South);
                                Ok(())
                            } else if nested_meta.path.is_ident("West") {
                                outputs.push(Side::West);
                                Ok(())
                            } else {
                                Err(meta.error("unsupported side"))
                            }
                        })?;

                        Ok(())
                    } else if meta.path.is_ident("machine") {
                        let value = meta.value()?;
                        let machine_expr: Expr = value.parse()?;
                        machine = Some(machine_expr);

                        Ok(())
                    } else {
                        Err(meta.error("unsupported attribute"))
                    }
                });

                if let Err(err) = parsing_result {
                    return err.to_compile_error().into();
                }

                let Some(texture_index) = texture_index else {
                    return syn::Error::new(attr.span(), "missing `texture`")
                        .into_compile_error()
                        .into();
                };

                let Some(machine) = machine else {
                    return syn::Error::new(attr.span(), "missing `machine`")
                        .into_compile_error()
                        .into();
                };

                variants.push(Variant::new(
                    name.clone(),
                    if inputs.is_empty() {
                        None
                    } else {
                        Some(inputs)
                    },
                    if outputs.is_empty() {
                        None
                    } else {
                        Some(outputs)
                    },
                    texture_index,
                    should_render,
                    machine,
                ));
            }
            None => {
                return syn::Error::new(variant.span(), "missing #[variant(...)] macro")
                    .into_compile_error()
                    .into();
            }
        }
    }

    let variants_len = variants.len();
    let variant_inputs_count = variants.iter().fold(0, |acc, variant| {
        if variant.inputs.is_some() {
            acc + 1
        } else {
            acc
        }
    });

    let get_input_sides = if variant_inputs_count == variants_len {
        let mut matches = Vec::new();

        for variant in &variants {
            let mut sides = Vec::new();

            for side in variant
                .inputs
                .as_ref()
                .expect("This is true, checked earlier")
            {
                sides.push(match side {
                    Side::North => quote! {crate::machines::Side::North},
                    Side::East => quote! {crate::machines::Side::East},
                    Side::South => quote! {crate::machines::Side::South},
                    Side::West => quote! {crate::machines::Side::West},
                });
            }

            let name = &variant.name;

            matches.push(quote! {
                Self::#name => vec![#(#sides),*],
            });
        }

        quote! {
            Some(match self {
                #(#matches)*
            })
        }
    } else if variant_inputs_count == 0 {
        quote! {
            None
        }
    } else {
        return syn::Error::new_spanned(ast, "Either all or no variants must have a texture index")
            .to_compile_error()
            .into();
    };

    let variant_outputs_count = variants.iter().fold(0, |acc, variant| {
        if variant.outputs.is_some() {
            acc + 1
        } else {
            acc
        }
    });

    let get_output_sides = if variant_outputs_count == variants_len {
        let mut matches = Vec::new();

        for variant in &variants {
            let mut sides = Vec::new();

            for side in variant
                .outputs
                .as_ref()
                .expect("This is true, checked earlier")
            {
                sides.push(match side {
                    Side::North => quote! {crate::machines::Side::North},
                    Side::East => quote! {crate::machines::Side::East},
                    Side::South => quote! {crate::machines::Side::South},
                    Side::West => quote! {crate::machines::Side::West},
                });
            }

            let name = &variant.name;

            matches.push(quote! {
                Self::#name => vec![#(#sides),*],
            });
        }

        quote! {
            Some(match self {
                #(#matches)*
            })
        }
    } else if variant_outputs_count == 0 {
        quote! {
            None
        }
    } else {
        return syn::Error::new_spanned(ast, "Either all or no variants must have a texture index")
            .to_compile_error()
            .into();
    };

    let should_render = {
        let mut matches = Vec::new();

        for variant in &variants {
            let should_render = &variant.should_render;
            let variant_name = &variant.name;

            matches.push(quote! {
                Self::#variant_name => #should_render,
            });
        }

        quote! {
            match self {
                #(#matches)*
            }
        }
    };

    let machine_variant_to_texture_index = {
        let mut matches = Vec::new();

        for variant in &variants {
            let texture_index = &variant.texture;
            let variant_name = &variant.name;

            matches.push(quote! {
                #name::#variant_name => #texture_index,
            });
        }

        quote! {
            bevy_ecs_tilemap::tiles::TileTextureIndex(match value {
                #(#matches)*
            })
        }
    };

    let texture_index_to_machine_variant = {
        let mut matches = Vec::new();

        for variant in &variants {
            let texture_index = &variant.texture;
            let variant_name = &variant.name;

            matches.push(quote! {
                #texture_index => #name::#variant_name,
            });
        }

        quote! {
            match value.0 {
                #(#matches)*
                _ => panic!("Can't convert {:?} to a {}!", value.0, #name_str),
            }
        }
    };

    let to_machine = {
        let mut matches = Vec::new();

        for variant in &variants {
            let machine = &variant.machine;
            let variant_name = &variant.name;

            matches.push(quote! {
                #name::#variant_name => #machine,
            });
        }

        quote! {
            match value {
                #(#matches)*
            }
        }
    };

    quote! {
        impl MachineVariants<#machine_type> for #name {
            fn get_input_sides(&self) -> Option<Vec<Side>> {
                #get_input_sides
            }

            fn get_output_sides(&self) -> Option<Vec<Side>> {
                #get_output_sides
            }

            fn should_render_item(&self) -> bool {
                #should_render
            }
        }

        impl From<#name> for #machine_type {
            fn from(value: #name) -> #machine_type {
                #to_machine
            }
        }

        impl From<bevy_ecs_tilemap::tiles::TileTextureIndex> for #name {
            fn from(value: bevy_ecs_tilemap::tiles::TileTextureIndex) -> Self {
                #texture_index_to_machine_variant
            }
        }

        impl From<#name> for bevy_ecs_tilemap::tiles::TileTextureIndex {
            fn from(value: #name) -> Self {
                #machine_variant_to_texture_index
            }
        }

    }
    .into()
}
