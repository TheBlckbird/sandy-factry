use crate::content::{
    machine_types::Side,
    machines::{
        belt::Belt, chest::Chest, combiner::Combiner, crafter::Crafter, furnace::Furnace,
        miner::Miner, splitter::Splitter,
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

    #[variant(inputs(North, East, South, West), outputs(North, East, South, West), texture = 13, machine = Chest::new())]
    Chest,

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

pub type MachineIndex = usize;
pub type VariantIndex = usize;

pub struct MachineGroup {
    thumbnail: ForegroundObject,
    variants: Vec<ForegroundObject>,
    is_standard_rotatable: bool,
}

impl MachineGroup {
    fn new(
        thumbnail: ForegroundObject,
        variants: Vec<ForegroundObject>,
        is_rotatable: bool,
    ) -> Self {
        Self {
            thumbnail,
            variants,
            is_standard_rotatable: is_rotatable,
        }
    }
}

impl ForegroundObject {
    /// Gets the machine and variant indices for selecting the current building.
    pub fn get_machine_indices(&self) -> (MachineIndex, VariantIndex) {
        Self::get_groups()
            .iter()
            .enumerate()
            .find_map(|(index, machine_group)| {
                machine_group
                    .variants
                    .iter()
                    .position(|variant| variant == self)
                    .map(|variant_index| (index, variant_index))
            })
            .expect("All foreground objects should be in a group")
    }

    /// Groups the variants of the machines together, always defining
    /// one variant that can be used as a thumbnail for a group
    fn get_groups() -> Vec<MachineGroup> {
        vec![
            MachineGroup::new(
                Self::BeltUp,
                vec![
                    Self::BeltDown,
                    Self::BeltLeft,
                    Self::BeltUp,
                    Self::BeltRight,
                ],
                true,
            ),
            MachineGroup::new(
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
            MachineGroup::new(
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
            MachineGroup::new(
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
            MachineGroup::new(
                Self::MinerDown,
                vec![
                    Self::MinerDown,
                    Self::MinerLeft,
                    Self::MinerUp,
                    Self::MinerRight,
                ],
                true,
            ),
            MachineGroup::new(
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
            MachineGroup::new(
                Self::CrafterDown,
                vec![
                    Self::CrafterDown,
                    Self::CrafterLeft,
                    Self::CrafterUp,
                    Self::CrafterRight,
                ],
                true,
            ),
            MachineGroup::new(
                Self::TunnelInUp,
                vec![
                    Self::TunnelInDown,
                    Self::TunnelInLeft,
                    Self::TunnelInUp,
                    Self::TunnelInRight,
                ],
                true,
            ),
            MachineGroup::new(
                Self::TunnelOutUp,
                vec![
                    Self::TunnelOutDown,
                    Self::TunnelOutLeft,
                    Self::TunnelOutUp,
                    Self::TunnelOutRight,
                ],
                true,
            ),
            MachineGroup::new(Self::Chest, vec![Self::Chest], false),
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

        Some(ForegroundObject::get_groups()[self.machine_index?].variants[variant_index])
    }

    /// Deselect the current machine.
    pub fn deselect(&mut self) {
        self.machine_index = None;
    }

    /// Whether there is currently a selected machine.
    pub fn is_something_selected(&self) -> bool {
        self.machine_index.is_some()
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
    ///
    /// `n` starts at 1 instead of 0.
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

    /// Select the nth variant of the current machine.
    ///
    /// `n` starts at 1 instead of 0.
    ///
    /// Panics if the variant index is too high and does nothing if no machine is selected.
    pub fn select_nth_variant(&mut self, mut n: usize) {
        if let Some(machine_index) = self.machine_index {
            n -= 1;

            let current_machine_variants = &ForegroundObject::get_groups()[machine_index].variants;

            let mut is_variant_existent = false;

            if self.is_standard_rotatable(machine_index) {
                if n < 4 {
                    self.standard_rotatable_variant_index = n;
                    is_variant_existent = true;
                }
            } else if n < current_machine_variants.len() {
                self.variant_indices[machine_index] = n;
                is_variant_existent = true;
            }

            if !is_variant_existent {
                panic!(
                    "Variant {n} doesn't exist for machine {:?}",
                    ForegroundObject::get_groups()[machine_index].thumbnail
                );
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
                == ForegroundObject::get_groups()[machine_index].variants.len() - 1
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
                    ForegroundObject::get_groups()[machine_index].variants.len() - 1;
            } else {
                self.variant_indices[machine_index] -= 1;
            }
        }
    }

    /// Whether this machien has four rotation variants.
    ///
    /// This is used for unifying rotation between different machines.
    fn is_standard_rotatable(&self, machine_index: usize) -> bool {
        ForegroundObject::get_groups()[machine_index].is_standard_rotatable
    }
}

impl Default for CurrentMachine {
    fn default() -> Self {
        Self {
            all_machines: ForegroundObject::get_groups()
                .iter()
                .map(|machine_group| machine_group.thumbnail)
                .collect(),
            machine_index: None,
            variant_indices: vec![0; ForegroundObject::get_groups().len()],
            standard_rotatable_variant_index: 0,
        }
    }
}
