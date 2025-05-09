use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use build_graph::build_graph;
use graph_to_world::graph_to_world;
use petgraph::prelude::*;
use simulate::simulate;

use crate::machines::Machine;

mod build_graph;
mod graph_to_world;
mod simulate;

#[derive(Resource, Default)]
struct SimulationGraph(Graph<(Machine, TilePos), ()>);

#[derive(Resource)]
struct SimulationTimer(Timer);

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationGraph>()
            .insert_resource(SimulationTimer(Timer::from_seconds(
                0.1,
                TimerMode::Repeating,
            )))
            .add_systems(FixedUpdate, (build_graph, simulate, graph_to_world).chain());
    }
}
