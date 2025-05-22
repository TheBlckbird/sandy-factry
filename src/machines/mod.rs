use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use std::{collections::VecDeque, fmt::Debug};

use crate::{Direction, plugins::world::MiddlegroundObject};

pub mod belt;
pub mod combiner;
pub mod crafter;
pub mod miner;

#[derive(Debug, Component)]
pub struct Machine {
    pub machine_type: Box<dyn MachineType>,
    pub input_items: InputItems,
    pub output_items: OutputItems,
}

impl Machine {
    pub fn new(
        machine_type: Box<dyn MachineType>,
        input_items: InputItems,
        output_items: OutputItems,
    ) -> Self {
        Self {
            machine_type,
            input_items,
            output_items,
        }
    }

    pub fn perform_action(&mut self, middleground_object: Option<MiddlegroundObject>) {
        self.machine_type.perform_action(
            &mut self.input_items,
            &mut self.output_items,
            middleground_object,
        );
    }
}

pub trait MachineType: Debug + Send + Sync {
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        output_items: &mut OutputItems,
        middleground_object: Option<MiddlegroundObject>,
    );
    fn clone_box(&self) -> Box<dyn MachineType>;
    fn can_accept(
        &self,
        item: &Item,
        input_items: &InputItems,
        output_items: &OutputItems,
        input_side: &Side,
    ) -> bool;
}

pub type OutputItems = VecDeque<Item>;
pub type Side = Direction;

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum Item {
    Coal,
    RawCopper,
    RawIron,
    CopperIngot,
    IronIngot,
}

impl From<Item> for TileTextureIndex {
    fn from(value: Item) -> Self {
        TileTextureIndex(match value {
            Item::Coal => 0,
            Item::RawCopper => 1,
            Item::RawIron => 2,
            Item::CopperIngot => 3,
            Item::IronIngot => 4,
        })
    }
}

pub trait MachineVariants<M: MachineType>:
    From<TileTextureIndex> + Into<TileTextureIndex> + Into<M> + Default
{
    fn get_input_sides(&self) -> Option<Vec<Side>>;
    fn get_output_sides(&self) -> Option<Vec<Side>>;
    fn should_render_item(&self) -> bool;
}

pub type InputItemsPart = Option<VecDeque<Item>>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InputItems {
    pub north: InputItemsPart,
    pub east: InputItemsPart,
    pub south: InputItemsPart,
    pub west: InputItemsPart,
}

impl InputItems {
    pub fn new(
        north: InputItemsPart,
        east: InputItemsPart,
        south: InputItemsPart,
        west: InputItemsPart,
    ) -> Self {
        Self {
            north,
            east,
            south,
            west,
        }
    }

    /// Gets exactly one input side
    /// Panics if zero or more than one sides are set
    pub fn exactly_one(&self) -> &VecDeque<Item> {
        let sides = [&self.north, &self.east, &self.south, &self.west];

        sides
            .into_iter()
            .filter_map(|direction| direction.as_ref())
            .exactly_one()
            .expect("There should be exactly one valid input")
    }

    /// Gets exactly one mutable reference to an input side
    /// Panics if zero or more than one sides are set
    pub fn exactly_one_mut(&mut self) -> &mut VecDeque<Item> {
        let sides = [
            &mut self.north,
            &mut self.east,
            &mut self.south,
            &mut self.west,
        ];

        sides
            .into_iter()
            .filter_map(|direction| direction.as_mut())
            .exactly_one()
            .expect("There should be exactly one valid input")
    }

    /// Gets a specific input side
    pub fn get_side(&self, side: &Side) -> &InputItemsPart {
        match side {
            Direction::North => &self.north,
            Direction::East => &self.east,
            Direction::South => &self.south,
            Direction::West => &self.west,
        }
    }

    /// Gets a mutable reference to a specific input side
    pub fn get_side_mut(&mut self, side: &Side) -> &mut InputItemsPart {
        match side {
            Direction::North => &mut self.north,
            Direction::East => &mut self.east,
            Direction::South => &mut self.south,
            Direction::West => &mut self.west,
        }
    }

    /// Returns the count of all items in all fields together
    pub fn count(&self) -> usize {
        let directions = [&self.north, &self.east, &self.south, &self.west];
        let mut count = 0;

        for direction in directions.iter().flat_map(|&direction| direction) {
            count += direction.len();
        }

        count
    }

    /// Gets all items
    /// Returns a Queue with north being at the beginning, then east, south and west
    pub fn all(&self) -> VecDeque<&Item> {
        let sides = [&self.north, &self.east, &self.south, &self.west];

        sides
            .iter()
            .flat_map(|side| side.iter().flat_map(|items| items.iter()))
            .collect()
    }
}

impl From<Option<Vec<Direction>>> for InputItems {
    fn from(value: Option<Vec<Direction>>) -> Self {
        let mut output = Self::default();

        for direction in value.iter().flatten() {
            match direction {
                Direction::North => output.north = Some(VecDeque::new()),
                Direction::East => output.east = Some(VecDeque::new()),
                Direction::South => output.south = Some(VecDeque::new()),
                Direction::West => output.west = Some(VecDeque::new()),
            }
        }

        output
    }
}
