use std::collections::{HashSet, VecDeque};

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};
use petgraph::{algo::tarjan_scc, prelude::*};

use crate::{
    machines::Side,
    plugins::world::{Middleground, MiddlegroundObject},
};

use super::SimulationGraph;

/// Do a single simulation step of the world based on the `SimulationGraph`
pub fn simulate(
    mut simulation_graph: ResMut<SimulationGraph>,
    tile_query: Query<(&TilePos, &TileTextureIndex), With<Middleground>>,
) {
    // Return if the simulation graph is empty aka there are no machines in the world
    if simulation_graph.node_count() == 0 {
        return;
    }

    // Get all the SCCs (Strongly Connected Components) using Tarjan's algorithm
    // This function also performs a topological sort on the result
    let scc = tarjan_scc(&**simulation_graph);

    let mut visited = HashSet::new();
    let mut times_machines_hit: HashMap<NodeIndex, u32> = HashMap::new();

    // Loop through all the first nodes of the SCCs
    for scc_start_node in scc.iter().map(|component| component[0]) {
        let mut next_nodes = VecDeque::from([scc_start_node]);

        // Run the BFS while there are nodes in the queue
        while let Some(node_index) = next_nodes.pop_front() {
            if visited.contains(&node_index) {
                continue;
            }

            // Get all the indices of the machines, we could theoretically push to
            let next_machine_indices: Vec<(NodeIndex, Side)> = simulation_graph
                .edges_directed(node_index, Direction::Outgoing)
                .map(|next_machine_edge| (next_machine_edge.target(), *next_machine_edge.weight()))
                .collect();

            let next_machine_indices_len = next_machine_indices.len(); // The value needs to be copied, because else the borrow checker would complain

            // Check whether there are even any machines to try to push to
            if !next_machine_indices.is_empty() {
                // Either increase the value this machine has been hit or insert one
                times_machines_hit
                    .entry(node_index)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);

                let times_machine_hit = times_machines_hit
                    .get(&node_index)
                    .expect("This was just inserted/updated, so it should exist");

                // If the machine hasn't been hit the amount of outputs it has, continue,
                // because we don't want to process it before it has been hit by all its outputs
                if *times_machine_hit != next_machine_indices_len as u32 {
                    continue;
                }

                // Insert all neighbors we want to visit into the queue
                for adjacent_node in
                    simulation_graph.neighbors_directed(node_index, Direction::Incoming)
                {
                    if !visited.contains(&adjacent_node) {
                        next_nodes.push_back(adjacent_node);
                    }
                }

                // Go through all machines the current machine could try to push to
                for (i, (next_machine_index, input_side)) in
                    next_machine_indices.into_iter().enumerate()
                {
                    // The output side of the connected machine is the opposite of the current machine's input side
                    let output_side = input_side.get_opposite();

                    // Retrieve the nodes of the current and connected machine
                    // This can't be done earlier, because of the borrow checker
                    let ((machine, machine_tile_pos), (next_machine, _)) =
                        simulation_graph.index_twice_mut(node_index, next_machine_index);

                    // Check whether this is the first time this loop is being run
                    // If this check wasn't made, the machine's action would be performed multiple times per frame
                    // The check has to be done in the loop, because of borrow checker rules (`index_twice_mut` is the problem)
                    // But this is the same as if it was done right before the loop
                    if i == 0 {
                        visited.insert(node_index);

                        // Perform the machine's action
                        machine
                            .perform_action(get_middleground_object(&tile_query, machine_tile_pos));
                    }

                    // Get the output items of the side being currently checked
                    let output_items = machine
                        .output_items
                        .get_side_mut(&output_side)
                        .unwrap_or_else(|| {
                            panic!("The side {output_side:?} should exist on this machine")
                        });

                    // Get the frontmost output item, if it exists
                    let Some(item) = output_items.front() else {
                        continue;
                    };

                    // If the connected machine can accept the item, push it to the correct input side
                    if next_machine.machine_type.can_accept(
                        item,
                        &next_machine.input_items,
                        &next_machine.output_items,
                        &input_side,
                    ) {
                        let item = output_items
                            .pop_front()
                            .expect("There should be an item in `output_items`");

                        next_machine
                            .input_items
                            .get_side_mut(&input_side)
                            .expect("The input side should be set; it's connected in the graph")
                            .push_back(item);
                    }
                }
            } else {
                // ... because if not, all the additional steps for trying to push items can be skipped

                // Insert all neighbors we want to visit into the queue
                for adjacent_node in
                    simulation_graph.neighbors_directed(node_index, Direction::Incoming)
                {
                    if !visited.contains(&adjacent_node) {
                        next_nodes.push_back(adjacent_node);
                    }
                }

                // Always mark this node as visited
                visited.insert(node_index);
                let (machine, machine_tile_pos) = &mut simulation_graph[node_index];

                // Perform the machine's action
                machine.perform_action(get_middleground_object(&tile_query, machine_tile_pos));
            }
        }
    }
}

/// Get the middleground object at `searched_tile_pos`
///
/// Returns `None` if there is no middleground object at that position
fn get_middleground_object(
    tile_query: &Query<(&TilePos, &TileTextureIndex), With<Middleground>>,
    searched_tile_pos: &mut TilePos,
) -> Option<MiddlegroundObject> {
    tile_query
        .iter()
        .find(|(tile_pos, _)| tile_pos == &searched_tile_pos)
        .and_then(|(_, tile_texture_index)| (*tile_texture_index).try_into().ok())
}
