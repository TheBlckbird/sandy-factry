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

// MARK: Plugin

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

// MARK: Schedule

/// A schedule that runs on every simulation tick (10 times per second)
#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct SimulationUpdate;

// MARK: Resources

#[derive(Resource, Default, Deref, DerefMut)]
struct SimulationGraph(Graph<(Machine, TilePos), Side>);

#[derive(Resource, Deref, DerefMut)]
struct SimulationTimer(Timer);

// MARK: Systems

fn setup(mut commands: Commands) {
    commands.init_resource::<SimulationGraph>();
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<SimulationGraph>();
}

/// Tick the [SimulationUpdate] schedule every tenth of a second
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
