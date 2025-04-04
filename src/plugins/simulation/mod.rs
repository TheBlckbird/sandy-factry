use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use petgraph::Graph;

use crate::buildings::TestBuilding;

#[derive(Resource)]
struct SimulationGraph(Graph<TestBuilding, ()>);

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}
