pub struct Service1 {
    incrementer: u32,
}

impl Service1 {
    pub fn new() -> Service1 {
        Service1 {
            incrementer: 0,
        }
    }

    pub fn increment(&mut self) {
        self.incrementer += 1;
    }
    
    pub fn value(&self) -> u32 {
        self.incrementer
    }
}