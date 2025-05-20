use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use build_graph::build_graph;
use graph_to_world::graph_to_world;
use petgraph::prelude::*;
use simulate::simulate;

use crate::machines::{Machine, Side};

use super::menu::GameState;

mod build_graph;
mod graph_to_world;
mod simulate;

#[derive(Resource, Default)]
struct SimulationGraph(Graph<(Machine, TilePos), Side>);

#[derive(Resource)]
struct SimulationTimer(Timer);

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), startup)
            .add_systems(
                FixedUpdate,
                (build_graph, simulate, graph_to_world)
                    .chain()
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

fn startup(mut commands: Commands) {
    commands.init_resource::<SimulationGraph>();
    commands.insert_resource(SimulationTimer(Timer::from_seconds(
        0.1,
        TimerMode::Repeating,
    )));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<SimulationGraph>();
    commands.remove_resource::<SimulationTimer>();
}
