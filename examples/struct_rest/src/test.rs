pub struct Test {
    pub field1: u8
}

impl Test {
    pub fn change_field1(&mut self, new_value: u8) {
        self.field1 = new_value;
    }

    pub fn get_field1(&self) -> u8 {
        return self.field1;
    }
}
