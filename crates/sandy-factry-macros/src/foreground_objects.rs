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
enum TunnelType {
    Input,
    Output,
}

#[derive(Debug)]
struct Variant {
    name: Ident,
    inputs: Option<Vec<Side>>,
    outputs: Option<Vec<Side>>,
    texture: LitInt,
    should_render: bool,
    machine: Expr,
    tunnel_type: Option<TunnelType>,
}

impl Variant {
    fn new(
        name: Ident,
        inputs: Option<Vec<Side>>,
        outputs: Option<Vec<Side>>,
        texture: LitInt,
        should_render: bool,
        machine: Expr,
        tunnel_type: Option<TunnelType>,
    ) -> Self {
        Self {
            name,
            inputs,
            outputs,
            texture,
            should_render,
            machine,
            tunnel_type,
        }
    }
}

pub fn impl_foreground_objects_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let enum_variants = if let Data::Enum(data_enum) = &ast.data {
        &data_enum.variants
    } else {
        return syn::Error::new_spanned(ast, "MachineVariants can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let mut variants = Vec::new();

    for variant in enum_variants {
        let name = &variant.ident;

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
                let mut inputs = Vec::new();
                let mut outputs = Vec::new();
                let mut texture_index = None;
                let mut machine: Option<Expr> = None;
                let mut should_render = false;
                let mut tunnel_type = None;

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
                    } else if meta.path.is_ident("tunnel") {
                        let value = meta.value()?;
                        let tunnel_value: Ident = value.parse()?;

                        match tunnel_value.to_string().as_str() {
                            "Input" => {
                                tunnel_type = Some(TunnelType::Input);
                                Ok(())
                            }
                            "Output" => {
                                tunnel_type = Some(TunnelType::Output);
                                Ok(())
                            }
                            _ => Err(meta.error("unsupported tunnel type")),
                        }
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

                let Some(ref machine) = machine else {
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
                    machine.clone(),
                    tunnel_type,
                ));
            }
            None => {
                return syn::Error::new(variant.span(), "missing #[variant(...)] macro")
                    .into_compile_error()
                    .into();
            }
        }
    }

    let get_input_sides = {
        let mut matches = Vec::new();

        for variant in &variants {
            let name = &variant.name;

            match variant.inputs.as_ref() {
                Some(input_sides) => {
                    let mut sides = Vec::new();

                    for side in input_sides {
                        sides.push(match side {
                            Side::North => quote! {crate::content::machine_types::Side::North},
                            Side::East => quote! {crate::content::machine_types::Side::East},
                            Side::South => quote! {crate::content::machine_types::Side::South},
                            Side::West => quote! {crate::content::machine_types::Side::West},
                        });
                    }

                    matches.push(quote! {
                        Self::#name => Some(vec![#(#sides),*]),
                    });
                }
                None => {
                    matches.push(quote! {
                        Self::#name => None,
                    });
                }
            }
        }

        quote! {
            match self {
                #(#matches)*
            }
        }
    };

    let get_output_sides = {
        let mut matches = Vec::new();

        for variant in &variants {
            let name = &variant.name;

            match variant.outputs.as_ref() {
                Some(output_sides) => {
                    let mut sides = Vec::new();

                    for side in output_sides {
                        sides.push(match side {
                            Side::North => quote! {crate::content::machine_types::Side::North},
                            Side::East => quote! {crate::content::machine_types::Side::East},
                            Side::South => quote! {crate::content::machine_types::Side::South},
                            Side::West => quote! {crate::content::machine_types::Side::West},
                        });
                    }

                    matches.push(quote! {
                        Self::#name => Some(vec![#(#sides),*]),
                    });
                }
                None => {
                    matches.push(quote! {
                        Self::#name => None,
                    });
                }
            }
        }

        quote! {
            match self {
                #(#matches)*
            }
        }
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

    let tunnel_type = {
        let mut matches = Vec::new();

        for variant in &variants {
            let tunnel_type = match &variant.tunnel_type {
                Some(tunnel_type) => match tunnel_type {
                    TunnelType::Input => {
                        quote! {Some(crate::content::machine_types::TunnelType::Input)}
                    }
                    TunnelType::Output => {
                        quote! {Some(crate::content::machine_types::TunnelType::Output)}
                    }
                },
                None => quote! {None},
            };

            let variant_name = &variant.name;

            matches.push(quote! {
                Self::#variant_name => #tunnel_type,
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

            if &variant_name.to_string() == "Nothing" {
                matches.push(quote! {
                    #name::#variant_name => Err("Building `Nothing` can't be converted to `TileTextureIndex`"),
                });
            } else {
                matches.push(quote! {
                    #name::#variant_name => Ok(bevy_ecs_tilemap::tiles::TileTextureIndex(#texture_index)),
                });
            }
        }

        quote! {
            match value {
                #(#matches)*
            }
        }
    };

    let texture_index_to_machine_variant = {
        let mut matches = Vec::new();

        for variant in &variants {
            let texture_index = &variant.texture;
            let variant_name = &variant.name;

            if &variant_name.to_string() != "Nothing" {
                matches.push(quote! {
                    #texture_index => #name::#variant_name,
                });
            }
        }

        quote! {
            match value.0 {
                #(#matches)*
                _ => panic!("Can't convert {:?} to `ForegroundObject`!", value.0),
            }
        }
    };

    let to_machine = {
        let mut matches = Vec::new();

        for variant in &variants {
            let machine = &variant.machine;
            let variant_name = &variant.name;

            if &variant_name.to_string() == "Nothing" {
                matches.push(quote! {
                    #name::#variant_name => Err("Building `Nothing` can't be converted to `ForegroundObject`"),
                })
            } else {
                matches.push(quote! {
                    #name::#variant_name => Ok(Box::new(#machine)),
                });
            }
        }

        quote! {
            match value {
                #(#matches)*
            }
        }
    };

    let select_next = {
        let mut matches = Vec::new();

        for (index, variant) in variants.iter().enumerate() {
            let mut next_index = index + 1;

            if next_index == variants.len() {
                next_index = 0;
            }

            let variant_name = &variant.name;
            let next_variant_name = &variants[next_index].name;

            matches.push(quote! {
                Self::#variant_name => Self::#next_variant_name,
            });
        }

        quote! {
            *self = match self {
                #(#matches)*
            };
        }
    };

    let select_prev = {
        let mut matches = Vec::new();

        for (index, variant) in variants.iter().enumerate() {
            let prev_index = if index == 0 {
                variants.len() - 1
            } else {
                index - 1
            };

            let variant_name = &variant.name;
            let prev_variant_name = &variants[prev_index].name;

            matches.push(quote! {
                Self::#variant_name => Self::#prev_variant_name,
            });
        }

        quote! {
            *self = match self {
                #(#matches)*
            };
        }
    };

    quote! {
        impl #name {
            pub fn get_input_sides(&self) -> Option<Vec<Side>> {
                #get_input_sides
            }

            pub fn get_output_sides(&self) -> Option<Vec<Side>> {
                #get_output_sides
            }

            pub fn should_render_item(&self) -> bool {
                #should_render
            }

            pub fn tunnel_type(&self) -> Option<crate::content::machine_types::TunnelType> {
                #tunnel_type
            }

            pub fn select_next(&mut self) {
                #select_next
            }

            pub fn select_previous(&mut self) {
                #select_prev
            }
        }

        impl TryFrom<#name> for Box<dyn crate::content::machine_types::MachineType> {
            type Error = &'static str;

            fn try_from(value: #name) -> Result<Box<dyn crate::content::machine_types::MachineType>, Self::Error> {
                #to_machine
            }
        }

        impl From<bevy_ecs_tilemap::tiles::TileTextureIndex> for #name {
            fn from(value: bevy_ecs_tilemap::tiles::TileTextureIndex) -> Self {
                #texture_index_to_machine_variant
            }
        }

        impl TryFrom<#name> for bevy_ecs_tilemap::tiles::TileTextureIndex {
            type Error = &'static str;

            fn try_from(value: #name) -> Result<Self, Self::Error> {
                #machine_variant_to_texture_index
            }
        }
    }
    .into()
}
