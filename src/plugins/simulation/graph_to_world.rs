use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use dyn_clone::clone_box;

use crate::content::machine_types::Machine;

use super::SimulationGraph;

/// Convert the changes from the graph back into the world
pub fn graph_to_world(
    simulation_graph: Res<SimulationGraph>,
    mut tile_query: Query<(&TilePos, &mut Machine)>,
) {
    // Go through every graph node
    for (machine, building_tile_pos) in simulation_graph
        .0
        .node_indices()
        .map(|node_index| &simulation_graph.0[node_index])
    {
        // Get the machine component
        let Some((_, mut machine_component)) = tile_query
            .iter_mut()
            .find(|&(tile_pos, _)| tile_pos == building_tile_pos)
        else {
            return;
        };

        // Clone the stats and items
        machine_component.input_items = machine.input_items.clone();
        machine_component.output_items = machine.output_items.clone();
        machine_component.machine_type = clone_box(&*machine.machine_type);
    }
}
