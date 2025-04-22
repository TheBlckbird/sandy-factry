use super::{BuildingType, Item};

#[derive(Debug, Clone, Copy)]
pub struct BeltSouthNorth;
impl BuildingType for BeltSouthNorth {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        println!("Reached a belt: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BeltNorthSouth;
impl BuildingType for BeltNorthSouth {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        println!("Reached a belt: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BeltEastWest;
impl BuildingType for BeltEastWest {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        println!("Reached a belt: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BeltSouthNorth;
impl BuildingType for BeltSouthNorth {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        println!("Reached a belt: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BeltSouthNorth;
impl BuildingType for BeltSouthNorth {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        println!("Reached a belt: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BeltSouthNorth;
impl BuildingType for BeltSouthNorth {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        println!("Reached a belt: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BeltSouthNorth;
impl BuildingType for BeltSouthNorth {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        println!("Reached a belt: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BeltSouthNorth;
impl BuildingType for BeltSouthNorth {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        println!("Reached a belt: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BeltSouthNorth;
impl BuildingType for BeltSouthNorth {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        println!("Reached a belt: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BeltSouthNorth;
impl BuildingType for BeltSouthNorth {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        println!("Reached a belt: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BeltSouthNorth;
impl BuildingType for BeltSouthNorth {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        println!("Reached a belt: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BeltSouthNorth;
impl BuildingType for BeltSouthNorth {
    fn perform_action(&self, contained_numbers: &[Item]) -> Result<Option<Item>, ()> {
        println!("Reached a belt: {}", contained_numbers.first().unwrap());
        Ok(None)
    }

    fn get_input_count(&self) -> usize {
        1
    }

    fn clone_box(&self) -> Box<(dyn BuildingType + 'static)> {
        Box::new(*self)
    }
}
