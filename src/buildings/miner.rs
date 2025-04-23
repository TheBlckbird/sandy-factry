use super::{BuildingType, Item};

#[derive(Debug, Clone, Copy)]
pub struct Miner;
impl BuildingType for Miner {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        todo!()
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}
