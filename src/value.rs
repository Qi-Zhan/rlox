pub type Value = f64;
#[derive(Debug, Clone)]
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn write_valuearray(&mut self, value: Value) {
        self.values.push(value);
    }
    
}
