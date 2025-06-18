use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_ecs_tilemap::{helpers::square_grid::neighbors::Neighbors, prelude::*};
use dyn_clone::clone_box;
use itertools::Itertools;
use petgraph::Graph;
use sandy_factry_helpers::graph::{add_edge_if_not_exists, get_or_create_node};

use crate::{
    Direction,
    content::machine_types::{Machine, TunnelType},
    plugins::{
        building::{
            BuildEvent, BuildingInput, BuildingOutput, foreground_objects::ForegroundObject,
        },
        world::MAP_SIZE,
    },
};

use super::SimulationGraph;

const MAXIMUM_TUNNEL_DISTANCE: u8 = 5;

/// Build a graph from the world representation
pub fn build_graph(
    mut _build_events: EventReader<BuildEvent>,
    tile_query: Query<(
        Entity,
        &TilePos,
        &TileTextureIndex,
        &BuildingInput,
        &BuildingOutput,
        &Machine,
    )>,
    mut simulation_graph: ResMut<SimulationGraph>,
) {
    // if build_events.is_empty() {
    //     return;
    // }

    // build_events.clear();

    // [FIXME] fix this in case the performance becomes a problem

    let mut factory_graph = Graph::new();

    if tile_query.is_empty() {
        simulation_graph.0 = factory_graph;
        return;
    }

    let mut visited = VecDeque::new();
    let mut next = VecDeque::new();
    let remaining_tiles: VecDeque<TilePos> = tile_query
        .iter()
        .map(|(_, &tile_pos, _, _, _, _)| tile_pos)
        .collect();

    let first_tile = tile_query.iter().next();

    match first_tile {
        Some(first_tile) => next.push_back(*first_tile.1),
        None => return,
    };

    for current_tile_pos in remaining_tiles {
        visited.push_back(current_tile_pos);

        let neighbors =
            Neighbors::get_square_neighboring_positions(&current_tile_pos, &MAP_SIZE, false);

        let (_, _, &tile_texture_index, tile_building_input, tile_building_output, tile_machine) =
            tile_query
                .iter()
                .find(|&(_, &tile_pos, _, _, _, _)| tile_pos == current_tile_pos)
                .expect(
                    "This tile should exist in the world because we got it from the world earlier.",
                );

        let tile_foreground_object = ForegroundObject::from(tile_texture_index);
        let output_sides = tile_foreground_object.get_output_sides();

        let building = Machine::new(
            clone_box(&*tile_machine.machine_type),
            tile_machine.input_items.clone(),
            tile_machine.output_items.clone(),
        );
        let current_node_index =
            get_or_create_node(&mut factory_graph, (building, &current_tile_pos));

        let add_neighbor = |neighbor: Option<TilePos>, next: &mut VecDeque<TilePos>| {
            if let Some(neighbor_pos) = neighbor
                && tile_query
                    .iter()
                    .any(&|(_, &tile_pos, _, _, _, _)| tile_pos == neighbor_pos)
                && !visited.contains(&neighbor_pos)
            {
                next.push_back(neighbor_pos);
            }
        };

        // Add neighbors
        add_neighbor(neighbors.north, &mut next);
        add_neighbor(neighbors.east, &mut next);
        add_neighbor(neighbors.south, &mut next);
        add_neighbor(neighbors.west, &mut next);

        let mut connect_inputs = false;
        let mut connect_outputs = false;
        let tunnel_type = tile_foreground_object.tunnel_type();

        match tunnel_type {
            Some(TunnelType::Input) => connect_inputs = true,
            Some(TunnelType::Output) => connect_outputs = true,
            None => {
                connect_inputs = true;
                connect_outputs = true;
            }
        }

        if connect_inputs {
            // Connect inputs
            for input in tile_building_input.iter().flatten() {
                let neighbor_pos = match input {
                    Direction::North => neighbors.north,
                    Direction::East => neighbors.east,
                    Direction::South => neighbors.south,
                    Direction::West => neighbors.west,
                };

                if let Some(neighbor_pos) = neighbor_pos
                    && let Some((
                        _,
                        _,
                        &neighbor_tile_texture_index,
                        _,
                        neighbor_building_output,
                        neighbor_machine,
                    )) = tile_query
                        .iter()
                        .find(|&(_, &tile_pos, _, _, _, _)| tile_pos == neighbor_pos)
                    && let Some(outputs) = neighbor_building_output.as_ref()
                    && outputs.iter().any(|output| output.get_opposite() == *input)
                    && !matches!(
                        ForegroundObject::from(neighbor_tile_texture_index).tunnel_type(),
                        Some(TunnelType::Input)
                    )
                {
                    let building = Machine::new(
                        clone_box(&*neighbor_machine.machine_type),
                        neighbor_machine.input_items.clone(),
                        neighbor_machine.output_items.clone(),
                    );
                    let new_node_index =
                        get_or_create_node(&mut factory_graph, (building, &neighbor_pos));
                    add_edge_if_not_exists(
                        &mut factory_graph,
                        new_node_index,
                        current_node_index,
                        *input,
                    );
                }
            }
        }

        if connect_outputs {
            // Connect outputs
            for output in tile_building_output.iter().flatten() {
                let neighbor_pos = match output {
                    Direction::North => neighbors.north,
                    Direction::East => neighbors.east,
                    Direction::South => neighbors.south,
                    Direction::West => neighbors.west,
                };

                if let Some(neighbor_pos) = neighbor_pos
                    && let Some((
                        _,
                        _,
                        &neighbor_tile_texture_index,
                        neighbor_building_input,
                        _,
                        neighbor_machine,
                    )) = tile_query
                        .iter()
                        .find(|&(_, &tile_pos, _, _, _, _)| tile_pos == neighbor_pos)
                    && let Some(inputs) = neighbor_building_input.0.as_ref()
                    && inputs.iter().any(|input| input.get_opposite() == *output)
                    && !matches!(
                        ForegroundObject::from(neighbor_tile_texture_index).tunnel_type(),
                        Some(TunnelType::Output)
                    )
                {
                    let building = Machine::new(
                        clone_box(&*neighbor_machine.machine_type),
                        neighbor_machine.input_items.clone(),
                        neighbor_machine.output_items.clone(),
                    );
                    let new_node_index =
                        get_or_create_node(&mut factory_graph, (building, &neighbor_pos));
                    add_edge_if_not_exists(
                        &mut factory_graph,
                        current_node_index,
                        new_node_index,
                        output.get_opposite(),
                    );
                }
            }
        }

        // Check if the current tile is a tunnel input
        if let Some(TunnelType::Input) = tunnel_type {
            // Get the output side of this tunnel
            let output_side = output_sides
                .as_ref()
                .expect("All tunnels should have an output")
                .iter()
                .exactly_one()
                .expect("This is Some, it definitely has a side inside");

            // convert the TilePos to a UVec2 for easier calculation
            let tile_pos_vec = UVec2::from(current_tile_pos);

            // Try all the different possible tunnel distances
            // This starts at one to avoid confusion when placing two tunnels right after each other
            for i in 1..=MAXIMUM_TUNNEL_DISTANCE {
                let searched_tile_pos = tile_pos_vec
                    .saturating_add_signed(output_side.as_ivec2() * i as i32)
                    .into();

                // Search through all other tiles to check if a matching tunnel output exists at the location searched for
                if let Some((_, tile_pos, _, building_input, _, machine)) = tile_query.iter().find(
                    |&(_, &tile_pos, &tile_texture_index, building_input, _, _)| {
                        tile_pos == searched_tile_pos
                            && *building_input
                                .as_ref()
                                .expect("All tunnels should have an input")
                                .iter()
                                .exactly_one()
                                .expect("This is Some, it definitely has a side inside")
                                == output_side.get_opposite()
                            && matches!(
                                ForegroundObject::from(tile_texture_index).tunnel_type(),
                                Some(TunnelType::Output)
                            )
                    },
                ) {
                    // If found, connect the two
                    let machine = Machine::new(
                        clone_box(&*machine.machine_type),
                        machine.input_items.clone(),
                        machine.output_items.clone(),
                    );
                    let new_node_index =
                        get_or_create_node(&mut factory_graph, (machine, tile_pos));
                    add_edge_if_not_exists(
                        &mut factory_graph,
                        current_node_index,
                        new_node_index,
                        *building_input
                            .as_ref()
                            .expect("All tunnels should have an input")
                            .iter()
                            .exactly_one()
                            .expect("This is Some, it definitely has a side inside"),
                    );

                    // Stop searching for more tunnel outputs after finding the nearest
                    break;
                }
            }
        }
    }

    **simulation_graph = factory_graph;
}
