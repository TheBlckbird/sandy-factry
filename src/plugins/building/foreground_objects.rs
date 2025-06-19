use crate::content::{
    machine_types::Side,
    machines::{
        belt::Belt, combiner::Combiner, crafter::Crafter, furnace::Furnace, miner::Miner,
        splitter::Splitter, void::Void,
    },
};

use bevy::prelude::*;
use sandy_factry_macros::ForegroundObjects;
use serde::{Deserialize, Serialize};

/// All the possible machines with all the possible variants.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ForegroundObjects)]
pub enum ForegroundObject {
    #[variant(inputs(South), outputs(North), texture = 0, machine = Belt, render = true)]
    BeltUp,
    #[variant(inputs(North), outputs(South), texture = 1, machine = Belt, render = true)]
    BeltDown,
    #[variant(inputs(West), outputs(East), texture = 2, machine = Belt, render = true)]
    BeltRight,
    #[variant(inputs(East), outputs(West), texture = 3, machine = Belt, render = true)]
    BeltLeft,

    #[variant(inputs(South), outputs(East), texture = 4, machine = Belt, render = true)]
    BeltDownRight,
    #[variant(inputs(West), outputs(South), texture = 5, machine = Belt, render = true)]
    BeltLeftDown,
    #[variant(inputs(North), outputs(West), texture = 6, machine = Belt, render = true)]
    BeltUpLeft,
    #[variant(inputs(East), outputs(North), texture = 7, machine = Belt, render = true)]
    BeltRightUp,
    #[variant(inputs(East), outputs(South), texture = 8, machine = Belt, render = true)]
    BeltRightDown,
    #[variant(inputs(South), outputs(West), texture = 9, machine = Belt, render = true)]
    BeltDownLeft,
    #[variant(inputs(West), outputs(North), texture = 10, machine = Belt, render = true)]
    BeltLeftUp,
    #[variant(inputs(North), outputs(East), texture = 11, machine = Belt, render = true)]
    BeltUpRight,

    #[variant(inputs(North), outputs(South), texture = 38, machine = Crafter::new())]
    CrafterDown,
    #[variant(inputs(East), outputs(West), texture = 39, machine = Crafter::new())]
    CrafterLeft,
    #[variant(inputs(South), outputs(North), texture = 40, machine = Crafter::new())]
    CrafterUp,
    #[variant(inputs(West), outputs(East), texture = 41, machine = Crafter::new())]
    CrafterRight,

    #[variant(outputs(South), texture = 34, machine = Miner::new())]
    MinerDown,
    #[variant(outputs(West), texture = 35, machine = Miner::new())]
    MinerLeft,
    #[variant(outputs(North), texture = 36, machine = Miner::new())]
    MinerUp,
    #[variant(outputs(East), texture = 37, machine = Miner::new())]
    MinerRight,

    #[variant(inputs(North, West), outputs(South), texture = 14, machine = Combiner::new([Side::North, Side::West]), render = true)]
    CombinerUpLeft,
    #[variant(inputs(West, South), outputs(East), texture = 15, machine = Combiner::new([Side::West, Side::South]), render = true)]
    CombinerLeftDown,
    #[variant(inputs(South, East), outputs(North), texture = 16, machine = Combiner::new([Side::South, Side::East]), render = true)]
    CombinerDownRight,
    #[variant(inputs(East, North), outputs(West), texture = 17, machine = Combiner::new([Side::East, Side::North]), render = true)]
    CombinerRightUp,
    #[variant(inputs(South, West), outputs(North), texture = 18, machine = Combiner::new([Side::South, Side::West]), render = true)]
    CombinerDownLeft,
    #[variant(inputs(West, North), outputs(East), texture = 19, machine = Combiner::new([Side::West, Side::North]), render = true)]
    CombinerLeftUp,
    #[variant(inputs(North, East), outputs(South), texture = 20, machine = Combiner::new([Side::North, Side::East]), render = true)]
    CombinerUpRight,
    #[variant(inputs(East, South), outputs(West), texture = 21, machine = Combiner::new([Side::East, Side::South]), render = true)]
    CombinerRightDown,

    #[variant(inputs(North), outputs(South, East), texture = 26, machine = Splitter::new([Side::South, Side::East]), render = true)]
    SplitterDownRight,
    #[variant(inputs(East), outputs(West, South), texture = 27, machine = Splitter::new([Side::West, Side::South]), render = true)]
    SplitterLeftDown,
    #[variant(inputs(South), outputs(North, West), texture = 28, machine = Splitter::new([Side::North, Side::West]), render = true)]
    SplitterUpLeft,
    #[variant(inputs(West), outputs(East, North), texture = 29, machine = Splitter::new([Side::East, Side::North]), render = true)]
    SplitterRightUp,
    #[variant(inputs(North), outputs(South, West), texture = 30, machine = Splitter::new([Side::South, Side::West]), render = true)]
    SplitterDownLeft,
    #[variant(inputs(East), outputs(West, North), texture = 31, machine = Splitter::new([Side::West, Side::North]), render = true)]
    SplitterLeftUp,
    #[variant(inputs(South), outputs(North, East), texture = 32, machine = Splitter::new([Side::North, Side::East]), render = true)]
    SplitterUpRight,
    #[variant(inputs(West), outputs(East, South), texture = 33, machine = Splitter::new([Side::East, Side::South]), render = true)]
    SplitterRightDown,

    #[variant(inputs(North, East, South, West), texture = 13, machine = Void)]
    Void,

    #[variant(inputs(North, West), outputs(South), texture = 42, machine = Furnace::new(Side::North, Side::West))]
    FurnaceUpLeft,
    #[variant(inputs(East, North), outputs(West), texture = 43, machine = Furnace::new(Side::East, Side::North))]
    FurnaceRightUp,
    #[variant(inputs(South, East), outputs(North), texture = 44, machine = Furnace::new(Side::South, Side::East))]
    FurnaceDownRight,
    #[variant(inputs(West, South), outputs(East), texture = 45, machine = Furnace::new(Side::West, Side::South))]
    FurnaceLeftDown,
    #[variant(inputs(North, East), outputs(South), texture = 46, machine = Furnace::new(Side::North, Side::East))]
    FurnaceUpRight,
    #[variant(inputs(East, South), outputs(West), texture = 47, machine = Furnace::new(Side::East, Side::South))]
    FurnaceRightDown,
    #[variant(inputs(South, West), outputs(North), texture = 48, machine = Furnace::new(Side::South, Side::West))]
    FurnaceDownLeft,
    #[variant(inputs(West, North), outputs(East), texture = 49, machine = Furnace::new(Side::West, Side::North))]
    FurnaceLeftUp,

    #[variant(inputs(North), outputs(South), texture = 50, machine = Belt, render = true, tunnel = Input)]
    TunnelInDown,
    #[variant(inputs(East), outputs(West), texture = 51, machine = Belt, render = true, tunnel = Input)]
    TunnelInLeft,
    #[variant(inputs(South), outputs(North), texture = 52, machine = Belt, render = true, tunnel = Input)]
    TunnelInUp,
    #[variant(inputs(West), outputs(East), texture = 53, machine = Belt, render = true, tunnel = Input)]
    TunnelInRight,

    #[variant(inputs(North), outputs(South), texture = 54, machine = Belt, render = true, tunnel = Output)]
    TunnelOutDown,
    #[variant(inputs(East), outputs(West), texture = 55, machine = Belt, render = true, tunnel = Output)]
    TunnelOutLeft,
    #[variant(inputs(South), outputs(North), texture = 56, machine = Belt, render = true, tunnel = Output)]
    TunnelOutUp,
    #[variant(inputs(West), outputs(East), texture = 57, machine = Belt, render = true, tunnel = Output)]
    TunnelOutRight,
}

impl ForegroundObject {
    /// Groups the variants of the machines together, always defining
    /// one variant that can be used as a thumbnail for a group
    fn get_groups() -> Vec<(Self, Vec<Self>, bool)> {
        vec![
            (
                Self::BeltUp,
                vec![
                    Self::BeltDown,
                    Self::BeltLeft,
                    Self::BeltUp,
                    Self::BeltRight,
                ],
                true,
            ),
            (
                Self::BeltDownRight,
                vec![
                    Self::BeltDownRight,
                    Self::BeltLeftDown,
                    Self::BeltUpLeft,
                    Self::BeltRightUp,
                    Self::BeltRightDown,
                    Self::BeltDownLeft,
                    Self::BeltLeftUp,
                    Self::BeltUpRight,
                ],
                false,
            ),
            (
                Self::CombinerDownLeft,
                vec![
                    Self::CombinerUpLeft,
                    Self::CombinerRightUp,
                    Self::CombinerDownRight,
                    Self::CombinerLeftDown,
                    Self::CombinerDownLeft,
                    Self::CombinerLeftUp,
                    Self::CombinerUpRight,
                    Self::CombinerRightDown,
                ],
                false,
            ),
            (
                Self::SplitterDownLeft,
                vec![
                    Self::SplitterDownRight,
                    Self::SplitterLeftDown,
                    Self::SplitterUpLeft,
                    Self::SplitterRightUp,
                    Self::SplitterDownLeft,
                    Self::SplitterRightDown,
                    Self::SplitterUpRight,
                    Self::SplitterLeftUp,
                ],
                false,
            ),
            (
                Self::MinerDown,
                vec![
                    Self::MinerDown,
                    Self::MinerLeft,
                    Self::MinerUp,
                    Self::MinerRight,
                ],
                true,
            ),
            (
                Self::FurnaceUpLeft,
                vec![
                    Self::FurnaceUpLeft,
                    Self::FurnaceRightUp,
                    Self::FurnaceDownRight,
                    Self::FurnaceLeftDown,
                    Self::FurnaceUpRight,
                    Self::FurnaceRightDown,
                    Self::FurnaceDownLeft,
                    Self::FurnaceLeftUp,
                ],
                false,
            ),
            (
                Self::CrafterDown,
                vec![
                    Self::CrafterDown,
                    Self::CrafterLeft,
                    Self::CrafterUp,
                    Self::CrafterRight,
                ],
                true,
            ),
            (
                Self::TunnelInUp,
                vec![
                    Self::TunnelInDown,
                    Self::TunnelInLeft,
                    Self::TunnelInUp,
                    Self::TunnelInRight,
                ],
                true,
            ),
            (
                Self::TunnelOutUp,
                vec![
                    Self::TunnelOutDown,
                    Self::TunnelOutLeft,
                    Self::TunnelOutUp,
                    Self::TunnelOutRight,
                ],
                true,
            ),
            (Self::Void, vec![Self::Void], false),
        ]
    }
}

// MARK: Resources

/// Holds information for the currently selected machine
/// and all the possible machine variants
#[derive(Resource, Clone)]
pub struct CurrentMachine {
    all_machines: Vec<ForegroundObject>,
    machine_index: Option<usize>,
    variant_indices: Vec<usize>,
    standard_rotatable_variant_index: usize,
}

impl CurrentMachine {
    const STANDARD_ROTATION_LENGTH: usize = 4;

    /// Get the currently selected [ForegroundObject]
    pub fn get_current_foreground_object(&self) -> Option<ForegroundObject> {
        let variant_index = if self.is_standard_rotatable(self.machine_index?) {
            self.standard_rotatable_variant_index
        } else {
            self.variant_indices[self.machine_index?]
        };

        Some(ForegroundObject::get_groups()[self.machine_index?].1[variant_index])
    }

    /// Deselect the current machine.
    pub fn deselect(&mut self) {
        self.machine_index = None;
    }

    /// Select the next machine.
    pub fn select_next_machine(&mut self) {
        match &self.machine_index {
            Some(machine_index) => {
                let mut next_index = machine_index + 1;

                if next_index == self.all_machines.len() {
                    next_index = 0;
                }

                self.machine_index = Some(next_index);
            }
            None => self.machine_index = Some(0),
        }
    }

    /// Select the nth machine, resetting the variant to the first one.
    pub fn select_nth_machine(&mut self, mut n: usize) {
        n -= 1;

        if n < self.all_machines.len() {
            match self.machine_index {
                Some(machine_index) if machine_index == n => {}
                _ => {
                    self.machine_index = Some(n);
                }
            }
        }
    }

    /// Select the previous machine, resetting the variant to the first one.
    pub fn select_prev_machine(&mut self) {
        match self.machine_index {
            Some(machine_index) => {
                if machine_index == 0 {
                    self.machine_index = Some(self.all_machines.len() - 1);
                } else {
                    self.machine_index = Some(machine_index - 1);
                }
            }
            None => self.machine_index = Some(0),
        }
    }

    /// Select the next variant of the current machine group.
    pub fn select_next_variant(&mut self) {
        if let Some(machine_index) = self.machine_index {
            if self.is_standard_rotatable(machine_index) {
                if self.standard_rotatable_variant_index == Self::STANDARD_ROTATION_LENGTH - 1 {
                    self.standard_rotatable_variant_index = 0;
                } else {
                    self.standard_rotatable_variant_index += 1;
                }
            } else if self.variant_indices[machine_index]
                == ForegroundObject::get_groups()[machine_index].1.len() - 1
            {
                self.variant_indices[machine_index] = 0;
            } else {
                self.variant_indices[machine_index] += 1;
            }
        }
    }

    /// Select the previous variant of the current machine group.
    pub fn select_prev_variant(&mut self) {
        if let Some(machine_index) = self.machine_index {
            if self.is_standard_rotatable(machine_index) {
                if self.standard_rotatable_variant_index == 0 {
                    self.standard_rotatable_variant_index = Self::STANDARD_ROTATION_LENGTH - 1;
                } else {
                    self.standard_rotatable_variant_index -= 1;
                }
            }
            if self.variant_indices[machine_index] == 0 {
                self.variant_indices[machine_index] =
                    ForegroundObject::get_groups()[machine_index].1.len() - 1;
            } else {
                self.variant_indices[machine_index] -= 1;
            }
        }
    }

    fn is_standard_rotatable(&self, machine_index: usize) -> bool {
        ForegroundObject::get_groups()[machine_index].2
    }
}

impl Default for CurrentMachine {
    fn default() -> Self {
        Self {
            all_machines: ForegroundObject::get_groups()
                .iter()
                .map(|(machine_icon, _, _)| *machine_icon)
                .collect(),
            machine_index: None,
            variant_indices: vec![0; ForegroundObject::get_groups().len()],
            standard_rotatable_variant_index: 0,
        }
    }
}
