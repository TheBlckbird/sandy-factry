use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_ecs_tilemap::{helpers::square_grid::neighbors::Neighbors, prelude::*};
use petgraph::{
    Graph,
    dot::{Config, Dot},
    graph::NodeIndex,
};

use crate::{Direction, MAP_SIZE};

use super::building::{BuildEvent, BuildingComponent, BuildingInput, BuildingOutput};

#[derive(Resource, Default)]
struct SimulationGraph(Graph<TilePos, ()>);

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationGraph>()
            .add_systems(Update, build_graph);
    }
}

fn build_graph(
    mut build_events: EventReader<BuildEvent>,
    tile_query: Query<
        (
            Entity,
            &TilePos,
            &TileTextureIndex,
            &BuildingInput,
            &BuildingOutput,
        ),
        With<BuildingComponent>,
    >,
    mut simulation_graph: ResMut<SimulationGraph>,
) {
    if build_events.is_empty() {
        return;
    }

    build_events.clear();

    let mut factory_graph = Graph::<TilePos, ()>::new();

    let mut visited: VecDeque<TilePos> = VecDeque::new();
    let mut next: VecDeque<TilePos> = VecDeque::new();
    let mut remaining_tiles: VecDeque<TilePos> = tile_query
        .iter()
        .map(|(_, &tile_pos, _, _, _)| tile_pos)
        .collect();

    if tile_query.is_empty() {
        return;
    }

    let first_tile = tile_query.iter().next();

    match first_tile {
        Some(first_tile) => next.push_back(*first_tile.1),
        None => return,
    };

    while !remaining_tiles.is_empty() {
        let current_tile_pos = remaining_tiles.pop_front().unwrap();
        visited.push_back(current_tile_pos);

        let neighbors =
            Neighbors::get_square_neighboring_positions(&current_tile_pos, &MAP_SIZE, false);

        let current_node_index = get_or_create_node(&mut factory_graph, &current_tile_pos);

        let tile = tile_query
            .iter()
            .find(|&(_, &tile_pos, _, _, _)| tile_pos == current_tile_pos)
            .unwrap();

        let add_neighbor = |neighbor: Option<TilePos>, next: &mut VecDeque<TilePos>| {
            if let Some(neighbor_pos) = neighbor {
                if tile_query
                    .iter()
                    .any(&|(_, &tile_pos, _, _, _)| tile_pos == neighbor_pos)
                    && !visited.contains(&neighbor_pos)
                {
                    next.push_back(neighbor_pos);
                }
            }
        };

        add_neighbor(neighbors.north, &mut next);
        add_neighbor(neighbors.east, &mut next);
        add_neighbor(neighbors.south, &mut next);
        add_neighbor(neighbors.west, &mut next);

        if let Some(input) = tile.3.0.as_ref() {
            let neighbor_pos = match input {
                Direction::North => neighbors.north,
                Direction::East => neighbors.east,
                Direction::South => neighbors.south,
                Direction::West => neighbors.west,
            };

            if let Some(neighbor_pos) = neighbor_pos {
                if tile_query
                    .iter()
                    .find(|&(_, &tile_pos, _, _, _)| tile_pos == neighbor_pos)
                    .and_then(|neighbor_tile| neighbor_tile.4.0.as_ref())
                    .filter(|neighbor_input| neighbor_input.get_opposite() == *input)
                    .is_some()
                {
                    let new_node_index = get_or_create_node(&mut factory_graph, &neighbor_pos);
                    add_edge_if_not_exists(&mut factory_graph, new_node_index, current_node_index);
                }
            }
        }

        if let Some(output) = tile.4.0.as_ref() {
            let neighbor_pos = match output {
                Direction::North => neighbors.north,
                Direction::East => neighbors.east,
                Direction::South => neighbors.south,
                Direction::West => neighbors.west,
            };

            if let Some(neighbor_pos) = neighbor_pos {
                if tile_query
                    .iter()
                    .find(|&(_, &tile_pos, _, _, _)| tile_pos == neighbor_pos)
                    .and_then(|neighbor_tile| neighbor_tile.3.0.as_ref())
                    .filter(|neighbor_input| neighbor_input.get_opposite() == *output)
                    .is_some()
                {
                    let new_node_index = get_or_create_node(&mut factory_graph, &neighbor_pos);
                    add_edge_if_not_exists(&mut factory_graph, current_node_index, new_node_index);
                }
            }
        }
    }

    println!(
        "{:?}",
        Dot::with_config(&factory_graph, &[Config::EdgeNoLabel])
    );

    simulation_graph.0 = factory_graph;
}

fn get_or_create_node(graph: &mut Graph<TilePos, ()>, tile_pos: &TilePos) -> NodeIndex {
    graph
        .node_indices()
        .find(|&node_index| tile_pos == &graph[node_index])
        .unwrap_or_else(|| graph.add_node(*tile_pos))
}

fn add_edge_if_not_exists(graph: &mut Graph<TilePos, ()>, a: NodeIndex, b: NodeIndex) {
    if graph.find_edge(a, b).is_none() {
        graph.add_edge(a, b, ());
    }
}
