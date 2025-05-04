use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};
use petgraph::{
    algo::{connected_components, tarjan_scc},
    dot::{Config, Dot},
    prelude::*,
};

use crate::plugins::world::{Middleground, MiddlegroundObject};

use super::SimulationGraph;

#[derive(Debug, Resource, Default)]
pub struct TicksPassed(pub u32);

pub fn simulate(
    mut simulation_graph: ResMut<SimulationGraph>,
    tile_query: Query<(&TilePos, &TileTextureIndex), With<Middleground>>,
    mut ticks_passed: ResMut<TicksPassed>,
) {
    // [TODO] remove debug
    if ticks_passed.0 <= 120 {
        ticks_passed.0 += 1;
        return;
    }

    ticks_passed.0 = 0;

    if simulation_graph.0.node_count() == 0 {
        return;
    }

    println!(
        "{:?}",
        Dot::with_config(&simulation_graph.0, &[Config::EdgeNoLabel])
    );

    let mut leaf_nodes: Vec<NodeIndex> = simulation_graph
        .0
        .externals(petgraph::Direction::Outgoing)
        .collect(); // [FIXME] This won't work anymore when the splitter is added

    let sub_graphs_count = connected_components(&simulation_graph.0);

    let mut additional_starting_nodes = Vec::with_capacity(sub_graphs_count - leaf_nodes.len());

    if leaf_nodes.len() < sub_graphs_count {
        tarjan_scc(&simulation_graph.0)
            .iter()
            .filter(|subgraph| {
                !subgraph
                    .iter()
                    .any(|subgraph_node| leaf_nodes.contains(subgraph_node))
            })
            .for_each(|sub_graph| additional_starting_nodes.push(*sub_graph.first().unwrap()));
    }

    leaf_nodes.append(&mut additional_starting_nodes);

    for leaf_node in leaf_nodes {
        simulation_graph.0.reverse();

        let mut bfs = Bfs::new(&simulation_graph.0, leaf_node);

        while let Some(node_index) = bfs.next(&simulation_graph.0) {
            let maybe_next_building_index = simulation_graph
                .0
                .edges_directed(node_index, Direction::Incoming)
                .next()
                .map(|next_building_edge| next_building_edge.source());

            let get_middleground_object = |searched_tile_pos| {
                // tile_query
                //     .iter()
                //     .find(|(tile_pos, _)| &searched_tile_pos == tile_pos)
                //     .and_then(|(_, tile_texture_index)| (*tile_texture_index).try_into().ok())

                Some(MiddlegroundObject::Coal) // [TODO] Get real tile, but chunking
            };

            match maybe_next_building_index {
                Some(next_building_index) => {
                    let ((building, building_tile_pos), (next_building, _)) = simulation_graph
                        .0
                        .index_twice_mut(node_index, next_building_index);

                    building.perform_action(get_middleground_object(building_tile_pos));

                    // let next_building_input_capacity =
                    //     next_building.building_type.get_input_count()
                    // - (next_building.input_items.len() + next_building.output_items.len());

                    let Some(item) = building.output_items.front() else {
                        continue;
                    };

                    if next_building.machine_type.can_accept(
                        item,
                        &next_building.input_items,
                        &next_building.output_items,
                    ) {
                        let Some(item) = building.output_items.pop_front() else {
                            continue; // this is technically redundant, but I don't want the game to crash, sooo...
                        };

                        next_building.input_items.push_back(item);
                    }
                }
                None => {
                    let (building, building_tile_pos) = &mut simulation_graph.0[node_index];

                    building.perform_action(get_middleground_object(building_tile_pos));
                }
            }
        }

        simulation_graph.0.reverse();
    }
}
