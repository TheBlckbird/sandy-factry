use std::ops::{Deref, DerefMut};

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use build_graph::build_graph;
use graph_to_world::graph_to_world;
use petgraph::prelude::*;
use simulate::simulate;

use crate::{
    content::machine_types::{Machine, Side},
    plugins::{interaction::game_not_paused, menu::GameState},
};

mod build_graph;
mod graph_to_world;
mod simulate;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_schedule(Schedule::new(SimulationUpdate))
            .insert_resource(SimulationTimer(Timer::from_seconds(
                0.1,
                TimerMode::Repeating,
            )))
            .add_systems(Main, tick_simulation_update)
            .add_systems(OnEnter(GameState::Game), setup)
            .add_systems(
                SimulationUpdate,
                (build_graph, simulate, graph_to_world)
                    .chain()
                    .run_if(game_not_paused),
            )
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

/// A schedule that runs on every simulation tick (10 times per second)
#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct SimulationUpdate;

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

fn setup(mut commands: Commands) {
    commands.init_resource::<SimulationGraph>();
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<SimulationGraph>();
}

fn tick_simulation_update(
    mut commands: Commands,
    mut simulation_timer: ResMut<SimulationTimer>,
    time: Res<Time>,
) {
    // If this timestep is a simulation tick, run the `SimulationUpdate` schedule
    if simulation_timer.tick(time.delta()).just_finished() {
        commands.run_schedule(SimulationUpdate);
    }
}
