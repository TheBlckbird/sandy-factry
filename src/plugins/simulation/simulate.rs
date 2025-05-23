use std::collections::{HashSet, VecDeque};

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};
use petgraph::{
    algo::{connected_components, tarjan_scc},
    dot::Dot,
    prelude::*,
};

use crate::{
    machines::Side,
    plugins::world::{Middleground, MiddlegroundObject},
};

use super::{SimulationGraph, SimulationTimer};

pub fn simulate(
    mut simulation_graph: ResMut<SimulationGraph>,
    tile_query: Query<(&TilePos, &TileTextureIndex), With<Middleground>>,
    mut simulation_timer: ResMut<SimulationTimer>,
    time: Res<Time>,
) {
    // Check if this tick is a simulation tick
    if !simulation_timer.tick(time.delta()).just_finished() {
        return;
    }

    // Return if the simulation graph is empty aka there are no machines in the world
    if simulation_graph.node_count() == 0 {
        return;
    }

    // println!(
    //     "{:?}",
    //     Dot::new(&simulation_graph.map(
    //         |_, (machine, tile_pos)| {
    //             let mut machine_type = format!("{:?}", machine.machine_type);
    //             machine_type = machine_type
    //                 .split(' ')
    //                 .next()
    //                 .expect("Whoops, no machine?")
    //                 .to_string();

    //             let tile_pos = format!("{}, {}", tile_pos.x, tile_pos.y);

    //             format!("{machine_type}; {tile_pos}")
    //         },
    //         |_, edge| edge,
    //     ))
    // );

    let mut leaf_nodes: Vec<NodeIndex> = simulation_graph
        .externals(petgraph::Direction::Outgoing)
        .collect();

    // [FIXME] circular refernces still won't work
    let sub_graphs_count = connected_components(&**simulation_graph);

    // let mut additional_starting_nodes = Vec::with_capacity(sub_graphs_count - leaf_nodes.len());
    let mut additional_starting_nodes = Vec::new();

    // if leaf_nodes.len() < sub_graphs_count {
    //     tarjan_scc(&simulation_graph)
    //         .iter()
    //         .filter(|subgraph| {
    //             !subgraph
    //                 .iter()
    //                 .any(|subgraph_node| leaf_nodes.contains(subgraph_node))
    //         })
    //         .for_each(|subgraph| {
    //             additional_starting_nodes.push(
    //                 *subgraph
    //                     .first()
    //                     .expect("There should be at least one subgraph"),
    //             )
    //         });
    // }

    let mut visited = VecDeque::new();

    leaf_nodes.append(&mut additional_starting_nodes);

    let mut times_loops_ran: HashMap<NodeIndex, u32> = HashMap::new();

    for leaf_node in leaf_nodes {
        simulation_graph.reverse();

        let mut bfs = Bfs::new(&**simulation_graph, leaf_node);

        while let Some(node_index) = bfs.next(&**simulation_graph) {
            // Problem: Splitter is being executed with the right argument but at the wrong time
            // at the second input, it's not the last, but the first machine to be executed...

            if visited.contains(&node_index) {
                break;
            }

            let next_building_indices: Vec<(NodeIndex, Side)> = simulation_graph
                .edges_directed(node_index, Direction::Incoming)
                .map(|next_building_edge| {
                    (next_building_edge.source(), *next_building_edge.weight())
                })
                .collect();

            let next_building_indices_len = next_building_indices.len();

            if !next_building_indices.is_empty() {
                times_loops_ran
                    .entry(node_index)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);

                let times_loop_ran = times_loops_ran
                    .get(&node_index)
                    .expect("This was just inserted/updated, so it should exist");

                if *times_loop_ran != next_building_indices_len as u32 {
                    break;
                }

                for (i, (next_building_index, input_side)) in
                    next_building_indices.into_iter().enumerate()
                {
                    let output_side = input_side.get_opposite();

                    let ((building, building_tile_pos), (next_building, next_building_tile_pos)) =
                        simulation_graph.index_twice_mut(node_index, next_building_index);

                    if i == 0 && *times_loop_ran == next_building_indices_len as u32 {
                        visited.push_back(node_index);
                        building.perform_action(get_middleground_object(
                            &tile_query,
                            building_tile_pos,
                        ));
                    }

                    let Some(output_items) =
                        building.output_items.get_side_mut(&output_side).as_mut()
                    else {
                        continue;
                    };

                    let Some(item) = output_items.front() else {
                        continue;
                    };

                    if next_building.machine_type.can_accept(
                        item,
                        &next_building.input_items,
                        &next_building.output_items,
                        &input_side,
                    ) {
                        let Some(item) = output_items.pop_front() else {
                            continue;
                        };

                        next_building
                            .input_items
                            .get_side_mut(&input_side)
                            .as_mut()
                            .expect("The input side should be set; it's connected in the graph")
                            .push_back(item);
                    }
                }
            } else {
                visited.push_back(node_index);
                let (building, building_tile_pos) = &mut simulation_graph[node_index];

                building.perform_action(get_middleground_object(&tile_query, building_tile_pos));
            }
        }

        simulation_graph.reverse();
    }
}

fn get_middleground_object(
    tile_query: &Query<(&TilePos, &TileTextureIndex), With<Middleground>>,
    searched_tile_pos: &mut TilePos,
) -> Option<MiddlegroundObject> {
    tile_query
        .iter()
        .find(|(tile_pos, _)| tile_pos == &searched_tile_pos)
        .and_then(|(_, tile_texture_index)| (*tile_texture_index).try_into().ok())
}
