type IntType = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Counter {
    value: IntType,
}

impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn get_value(&self) -> IntType {
        return self.value;
    }
    pub fn increment(&mut self) {
        self.value += 1
    }
    pub fn reset(&mut self) {
        self.value = 0
    }
}

impl From<Counter> for IntType {
    fn from(value: Counter) -> Self {
        value.get_value()
    }
}
