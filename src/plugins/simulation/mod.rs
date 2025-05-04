use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use build_graph::build_graph;
use graph_to_world::graph_to_world;
use petgraph::prelude::*;
use simulate::{TicksPassed, simulate};

use crate::machines::Machine;

mod build_graph;
mod graph_to_world;
mod simulate;

#[derive(Resource, Default)]
struct SimulationGraph(Graph<(Machine, TilePos), ()>);

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationGraph>()
            .init_resource::<TicksPassed>() // [TODO] remove this DEBUG
            .add_systems(Update, (build_graph, simulate, graph_to_world).chain());
    }
}
