use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::buildings::Building;

use super::SimulationGraph;

pub fn graph_to_world(
    simulation_graph: Res<SimulationGraph>,
    mut tile_query: Query<(&TilePos, &mut Building)>,
) {
    for (building, building_tile_pos) in simulation_graph
        .0
        .node_indices()
        .map(|node_index| &simulation_graph.0[node_index])
    {
        let Some((_, mut building_component)) = tile_query
            .iter_mut()
            .find(|&(tile_pos, _)| tile_pos == building_tile_pos)
        else {
            return;
        };

        building_component.input_items = building.input_items.clone();
        building_component.output_items = building.output_items.clone();
    }
}
