use std::ops::{Deref, DerefMut};

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use build_graph::build_graph;
use graph_to_world::graph_to_world;
use petgraph::prelude::*;
use simulate::simulate;

use crate::machines::{Machine, Side};

mod build_graph;
mod graph_to_world;
mod simulate;

#[derive(Resource, Default)]
struct SimulationGraph(Graph<(Machine, TilePos), Side>);

impl Deref for SimulationGraph {
    type Target = Graph<(Machine, TilePos), Side>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SimulationGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Resource)]
struct SimulationTimer(Timer);

impl Deref for SimulationTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SimulationTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationGraph>()
            .insert_resource(SimulationTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )))
            .add_systems(FixedUpdate, (build_graph, simulate, graph_to_world).chain());
    }
}
