use crate::machines::{
    Side, belt::Belt, combiner::Combiner, crafter::Crafter, miner::Miner, splitter::Splitter,
    void::Void,
};

use bevy::prelude::*;
use sandy_factry_macros::ForegroundObjects;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Resource, Default, Clone, Copy, PartialEq, Serialize, Deserialize, ForegroundObjects,
)]
pub enum ForegroundObject {
    #[default]
    #[variant(texture = -1, machine = Nothing)]
    Nothing,
    #[variant(inputs(South), outputs(North), texture = 0, machine = Belt, render = true)]
    BeltUp,
    #[variant(inputs(North), outputs(South), texture = 1, machine = Belt, render = true)]
    BeltDown,
    #[variant(inputs(East), outputs(West), texture = 2, machine = Belt, render = true)]
    BeltLeft,
    #[variant(inputs(West), outputs(East), texture = 3, machine = Belt, render = true)]
    BeltRight,

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

    #[variant(inputs(North, West), outputs(South), texture = 12, machine = Crafter::default())]
    Crafter,

    #[variant(inputs(North), outputs(South), texture = 13, machine = Miner)]
    Miner,

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
    #[variant(inputs(South), outputs(North, East), texture = 28, machine = Splitter::new([Side::North, Side::East]), render = true)]
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

    #[variant(inputs(North, East, South, West), texture = 34, machine = Void)]
    Void,
}
