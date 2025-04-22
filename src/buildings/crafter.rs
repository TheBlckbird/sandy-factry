use super::{BuildingType, Item};

#[derive(Debug, Clone, Copy)]
pub struct Crafter;
impl BuildingType for Crafter {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        println!("Reached an end: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}
