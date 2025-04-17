#[derive(Debug, Clone)]
pub struct Card {
    id: usize,
    value: usize,
    color: i32, //0=noir 1=rouge
}

impl Card {
    pub fn new(id: usize, value: usize, color: i32) -> Card {
        Card {
            id,
            value,
            color,
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

    pub fn get_color(&self) -> i32 {
        self.color
    }

    pub fn set_color(&mut self, color: usize) {}
}
