use super::{BuildingType, Item};

#[derive(Debug, Clone, Copy)]
pub struct Belt;
impl BuildingType for Belt {
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
