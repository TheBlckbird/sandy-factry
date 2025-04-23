use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::plugins::building::BuildingComponent;

use super::SimulationGraph;

pub fn graph_to_world(
    simulation_graph: Res<SimulationGraph>,
    mut tile_query: Query<(&TilePos, &mut BuildingComponent)>,
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

        building_component.items = building.items.clone();
    }
}
