use bevy::prelude::*;
use petgraph::{
    algo::{connected_components, tarjan_scc},
    prelude::*,
};

use super::SimulationGraph;

pub fn simulate(mut simulation_graph: ResMut<SimulationGraph>) {
    if simulation_graph.0.node_count() == 0 {
        return;
    }

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

            match maybe_next_building_index {
                Some(next_building_index) => {
                    let ((building, _), (next_building, _)) = simulation_graph
                        .0
                        .index_twice_mut(node_index, next_building_index);

                    building.perform_action();

                    let next_building_input_capacity =
                        next_building.building_type.get_input_count()
                            - (next_building.input_items.len() + next_building.output_items.len());

                    if next_building_input_capacity >= 1 {
                        let Some(item) = building.output_items.pop_front() else {
                            continue;
                        };

                        next_building.input_items.push_back(item);
                    }
                }
                None => {
                    let (building, _) = &mut simulation_graph.0[node_index];

                    building.perform_action();
                }
            }
        }

        simulation_graph.0.reverse();
    }
}
