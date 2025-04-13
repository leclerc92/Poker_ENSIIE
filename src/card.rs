#[derive(Debug, Clone)]
pub struct Card {
    id: usize,
    value: usize,
}

impl Card {
    pub fn new(id: usize, value: usize) -> Card {
        Card {
            id,
            value,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    }
}
