use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_ecs_tilemap::{helpers::square_grid::neighbors::Neighbors, prelude::*};
use dyn_clone::clone_box;
use petgraph::Graph;
use sandy_factry_helpers::graph::{add_edge_if_not_exists, get_or_create_node};

use crate::{Direction, content::machine_types::Machine, plugins::world::MAP_SIZE};

use crate::plugins::building::{BuildEvent, BuildingInput, BuildingOutput};

use super::SimulationGraph;

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

        let tile = tile_query
            .iter()
            .find(|&(_, &tile_pos, _, _, _, _)| tile_pos == current_tile_pos)
            .expect(
                "This tile should exist in the world because we got it from the world earlier.",
            );

        let building = Machine::new(
            clone_box(&*tile.5.machine_type),
            tile.5.input_items.clone(),
            tile.5.output_items.clone(),
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

        add_neighbor(neighbors.north, &mut next);
        add_neighbor(neighbors.east, &mut next);
        add_neighbor(neighbors.south, &mut next);
        add_neighbor(neighbors.west, &mut next);

        for input in tile.3.0.iter().flatten() {
            let neighbor_pos = match input {
                Direction::North => neighbors.north,
                Direction::East => neighbors.east,
                Direction::South => neighbors.south,
                Direction::West => neighbors.west,
            };

            if let Some(neighbor_pos) = neighbor_pos
                && let Some(neighbor_tile) = tile_query
                    .iter()
                    .find(|&(_, &tile_pos, _, _, _, _)| tile_pos == neighbor_pos)
                && let Some(outputs) = neighbor_tile.4.0.as_ref()
                && outputs.iter().any(|output| output.get_opposite() == *input)
            {
                let building = Machine::new(
                    clone_box(&*neighbor_tile.5.machine_type),
                    neighbor_tile.5.input_items.clone(),
                    neighbor_tile.5.output_items.clone(),
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

        for output in tile.4.0.iter().flatten() {
            let neighbor_pos = match output {
                Direction::North => neighbors.north,
                Direction::East => neighbors.east,
                Direction::South => neighbors.south,
                Direction::West => neighbors.west,
            };

            if let Some(neighbor_pos) = neighbor_pos
                && let Some(neighbor_tile) = tile_query
                    .iter()
                    .find(|&(_, &tile_pos, _, _, _, _)| tile_pos == neighbor_pos)
                && let Some(inputs) = neighbor_tile.3.0.as_ref()
                && inputs.iter().any(|input| input.get_opposite() == *output)
            {
                let building = Machine::new(
                    clone_box(&*neighbor_tile.5.machine_type),
                    neighbor_tile.5.input_items.clone(),
                    neighbor_tile.5.output_items.clone(),
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

    simulation_graph.0 = factory_graph;
}
