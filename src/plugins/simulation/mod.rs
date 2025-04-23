use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use build_graph::build_graph;
use graph_to_world::graph_to_world;
use petgraph::Graph;

use crate::buildings::Building;

mod build_graph;
mod graph_to_world;

#[derive(Resource, Default)]
struct SimulationGraph(Graph<(Building, TilePos), ()>);

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationGraph>()
            .add_systems(Update, (build_graph, graph_to_world));
    }
}
