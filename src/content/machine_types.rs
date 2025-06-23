use as_any::AsAny;
use bevy::prelude::*;
use dyn_clone::DynClone;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use std::{collections::VecDeque, fmt::Debug};

use crate::{
    Direction,
    content::items::{Item, ItemType},
    plugins::world::MiddlegroundObject,
};

// MARK: Machine
#[derive(Debug, Component, Serialize, Deserialize)]
pub struct Machine {
    pub machine_type: Box<dyn MachineType>,
    pub input_items: InputItems,
    pub output_items: Option<OutputItems>,
}

impl Machine {
    pub fn new(
        machine_type: Box<dyn MachineType>,
        input_items: InputItems,
        output_items: Option<OutputItems>,
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
            self.output_items.as_mut(),
            middleground_object,
        );
    }
}

// MARK: MachineType

/// The trait all machines have to implement
///
/// ## Example
///
/// ```rs
/// struct MyMachine;
///
/// #[typetag::serde]
/// impl MachineType for MyMachine {
///     fn perform_action(
///         &mut self,
///         input_items: &mut InputItems,
///         output_items: Option<&mut OutputItems>,
///         middleground_object: Option<MiddlegroundObject>,
///     ) {
///         todo!()
///     }
///
///     fn can_accept(
///        &self,
///        item: &Item,
///        input_items: &InputItems,
///        output_items: Option<&OutputItems>,
///        input_side: &Side,
///    ) -> bool {
///        todo!()
///    }
///
///     // optional, defaults to `false`
///     fn is_selectable(&self) -> bool {
///         todo!()
///     }
/// }
/// ```
#[typetag::serde(tag = "type")]
pub trait MachineType: Debug + Send + Sync + AsAny + DynClone {
    /// This method is executed every simulation step
    ///
    /// It should perform whatever action this building does.
    ///
    /// `input_items` are a queue for every input side of items that were pushed into this building by other machines..
    /// [MachineType::can_accept] controls the input flow of the items.
    ///
    /// `output_items` should contain all the items that are ready to be pushed to the next machine.
    ///
    /// `middleground_object` is the [MiddlegroundObject] currently directly under the machine, if there is one.
    fn perform_action(
        &mut self,
        input_items: &mut InputItems,
        output_items: Option<&mut OutputItems>,
        middleground_object: Option<MiddlegroundObject>,
    );

    /// Can accept controls whether a specific item can be inputted into the current machine.
    ///
    /// `item` is a reference to the item.
    ///
    /// `input_items` is a reference to the items that are already in
    /// this machine's input.
    ///
    /// `output_items` is a reference to all the items in this machine
    /// that are ready to be pushed to the next one.
    ///
    /// `input_side` is the side the item's tried to be inputted to.
    fn can_accept(
        &self,
        item: &ItemType,
        input_items: &InputItems,
        output_items: Option<&OutputItems>,
        input_side: &Side,
    ) -> bool;

    /// Whether the machine can be selected by left, to open a menu for example.
    ///
    /// This currently only defines whether the selection marker is shown or not,
    /// all the other logic is handled in other places.
    fn is_selectable(&self) -> bool {
        false
    }

    /// Whether this can be ticked multiple times a tick
    ///
    /// Should usually only be set to true on belts
    fn tick_after_first(&self) -> bool {
        false
    }
}

pub type Side = Direction;

// MARK: ItemsSet

pub type InputItems = ItemsSet;
pub type ItemsSetPart = Option<VecDeque<Item>>;

/// Datastructure containing a queue of items for up to all four directions
///
/// Used for specifying the input and output items
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemsSet {
    pub north: ItemsSetPart,
    pub east: ItemsSetPart,
    pub south: ItemsSetPart,
    pub west: ItemsSetPart,
}

impl ItemsSet {
    pub fn new(
        north: ItemsSetPart,
        east: ItemsSetPart,
        south: ItemsSetPart,
        west: ItemsSetPart,
    ) -> Self {
        Self {
            north,
            east,
            south,
            west,
        }
    }

    /// Gets exactly one input side.
    ///
    /// Panics if zero sides or more than one side is set.
    pub fn exactly_one(&self) -> &VecDeque<Item> {
        let sides = [&self.north, &self.east, &self.south, &self.west];

        sides
            .into_iter()
            .filter_map(|direction| direction.as_ref())
            .exactly_one()
            .expect("There should be exactly one valid input")
    }

    /// Gets exactly one input side as a mutable reference.
    ///
    /// Panics if zero sides or more than one side is set.
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

    /// Gets a specific input side.
    pub fn get_side(&self, side: &Side) -> Option<&VecDeque<Item>> {
        match side {
            Direction::North => &self.north,
            Direction::East => &self.east,
            Direction::South => &self.south,
            Direction::West => &self.west,
        }
        .as_ref()
    }

    /// Gets a mutable reference to a specific input side.
    pub fn get_side_mut(&mut self, side: &Side) -> Option<&mut VecDeque<Item>> {
        match side {
            Direction::North => &mut self.north,
            Direction::East => &mut self.east,
            Direction::South => &mut self.south,
            Direction::West => &mut self.west,
        }
        .as_mut()
    }

    /// Returns the count of all items in all fields together.
    pub fn count(&self) -> usize {
        let directions = [&self.north, &self.east, &self.south, &self.west];
        let mut count = 0;

        for direction in directions.iter().flat_map(|&direction| direction) {
            count += direction.len();
        }

        count
    }

    /// Returns the amount of this specific item in all fields together
    pub fn count_item(&self, counted_item: &ItemType) -> usize {
        let directions = [&self.north, &self.east, &self.south, &self.west];
        let mut count = 0;

        for direction in directions.iter().flat_map(|&direction| direction) {
            count += direction
                .iter()
                .filter(|&item| **item == *counted_item)
                .count();
        }

        count
    }

    /// Checks whether there are any items in any of the fields.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// Gets a reference to all items as a queue.
    ///
    /// Returns a Queue with north being at the beginning, then east, south and west.
    pub fn all(&self) -> VecDeque<&Item> {
        let sides = [&self.north, &self.east, &self.south, &self.west];

        sides
            .iter()
            .flat_map(|side| side.iter().flat_map(|items| items.iter()))
            .collect()
    }

    /// Gets a mutable reference all items as a queue.
    ///
    /// Returns a Queue with north being at the beginning, then east, south and west.
    pub fn all_mut(&mut self) -> VecDeque<&mut Item> {
        let sides = [
            &mut self.north,
            &mut self.east,
            &mut self.south,
            &mut self.west,
        ];

        sides
            .into_iter()
            .flat_map(|side| side.iter_mut().flat_map(|items| items.iter_mut()))
            .collect()
    }
}

impl From<Option<Vec<Direction>>> for ItemsSet {
    fn from(value: Option<Vec<Direction>>) -> Self {
        let mut output = Self::default();

        for direction in value.iter().flatten() {
            let side_to_set = match direction {
                Direction::North => &mut output.north,
                Direction::East => &mut output.east,
                Direction::South => &mut output.south,
                Direction::West => &mut output.west,
            };

            *side_to_set = Some(VecDeque::new());
        }

        output
    }
}

// MARK: OutputItems

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputItems {
    SingleSide(VecDeque<Item>),
    MultipleSides(PreferredItemsSide),
}

impl OutputItems {
    pub fn new_single_side() -> Self {
        Self::SingleSide(VecDeque::new())
    }

    pub fn new_multiple_sides() -> Self {
        Self::MultipleSides(PreferredItemsSide::new())
    }

    pub fn unwrap_single_side(&self) -> &VecDeque<Item> {
        let Self::SingleSide(items) = self else {
            panic!("Expected `OutputItems` to be `SingleSide`");
        };

        items
    }

    pub fn unwrap_single_side_mut(&mut self) -> &mut VecDeque<Item> {
        let Self::SingleSide(items) = self else {
            panic!("Expected `OutputItems` to be `SingleSide`");
        };

        items
    }

    pub fn unwrap_multiple_sides(&self) -> &PreferredItemsSide {
        let Self::MultipleSides(items) = self else {
            panic!("Expected `OutputItems` to be `MultipleSides`");
        };

        items
    }

    pub fn unwrap_multiple_sides_mut(&mut self) -> &mut PreferredItemsSide {
        let Self::MultipleSides(items) = self else {
            panic!("Expected `OutputItems` to be `MultipleSides`");
        };

        items
    }

    pub fn len(&self) -> usize {
        match self {
            Self::SingleSide(items) => items.len(),
            Self::MultipleSides(preferred_items_side) => preferred_items_side.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_items(&self) -> &VecDeque<Item> {
        match self {
            OutputItems::SingleSide(items) => items,
            OutputItems::MultipleSides(preferred_items_side) => &preferred_items_side.items,
        }
    }

    pub fn get_items_mut(&mut self) -> &mut VecDeque<Item> {
        match self {
            OutputItems::SingleSide(items) => items,
            OutputItems::MultipleSides(preferred_items_side) => &mut preferred_items_side.items,
        }
    }
}

impl TryFrom<Option<Vec<Side>>> for OutputItems {
    type Error = ();

    fn try_from(value: Option<Vec<Side>>) -> Result<Self, Self::Error> {
        match value.ok_or(())?.len() {
            0 => Err(()),
            1 => Ok(Self::new_single_side()),
            _ => Ok(Self::new_multiple_sides()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferredItemsSide {
    pub items: VecDeque<Item>,
    /// The preferred side, the output item should go to.
    ///
    /// Goes from most preferred at index 0 to least preferred side at the end.
    ///
    /// The item stays in the machine, if all sides are occupied.
    pub preferred_sides: Vec<Side>,
}

impl PreferredItemsSide {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
            preferred_sides: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn push_side(&mut self, side: Side) {
        self.preferred_sides.push(side)
    }
}

impl Default for PreferredItemsSide {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused)]
pub trait UnwrapOutputItems {
    fn unwrap_single_side(&self) -> &VecDeque<Item>;
    fn unwrap_multiple_sides(&self) -> &PreferredItemsSide;
}

#[allow(unused)]
pub trait UnwrapOutputItemsMut {
    fn unwrap_single_side_mut(&mut self) -> &mut VecDeque<Item>;
    fn unwrap_multiple_sides_mut(&mut self) -> &mut PreferredItemsSide;
}

impl UnwrapOutputItems for Option<&OutputItems> {
    fn unwrap_single_side(&self) -> &VecDeque<Item> {
        self.expect("Option<OutputItems> should be Some")
            .unwrap_single_side()
    }

    fn unwrap_multiple_sides(&self) -> &PreferredItemsSide {
        self.expect("Option<OutputItems> should be Some")
            .unwrap_multiple_sides()
    }
}

impl UnwrapOutputItemsMut for Option<&mut OutputItems> {
    fn unwrap_single_side_mut(&mut self) -> &mut VecDeque<Item> {
        self.as_deref_mut()
            .expect("Option<OutputItems> should be Some")
            .unwrap_single_side_mut()
    }

    fn unwrap_multiple_sides_mut(&mut self) -> &mut PreferredItemsSide {
        self.as_deref_mut()
            .expect("Option<OutputItems> should be Some")
            .unwrap_multiple_sides_mut()
    }
}

impl UnwrapOutputItems for Option<&mut OutputItems> {
    fn unwrap_single_side(&self) -> &VecDeque<Item> {
        self.as_deref()
            .expect("Option<OutputItems> should be Some")
            .unwrap_single_side()
    }

    fn unwrap_multiple_sides(&self) -> &PreferredItemsSide {
        self.as_deref()
            .expect("Option<OutputItems> should be Some")
            .unwrap_multiple_sides()
    }
}

// MARK: TunnelType

pub enum TunnelType {
    Input,
    Output,
}
