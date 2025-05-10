use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::machines::Machine;

use super::SimulationGraph;

pub fn graph_to_world(
    simulation_graph: Res<SimulationGraph>,
    mut tile_query: Query<(&TilePos, &mut Machine)>,
) {
    for (machine, building_tile_pos) in simulation_graph
        .0
        .node_indices()
        .map(|node_index| &simulation_graph.0[node_index])
    {
        let Some((_, mut machine_component)) = tile_query
            .iter_mut()
            .find(|&(tile_pos, _)| tile_pos == building_tile_pos)
        else {
            return;
        };

        machine_component.input_items = machine.input_items.clone();
        machine_component.output_items = machine.output_items.clone();
        machine_component.machine_type = machine.machine_type.clone_box();
    }
}
