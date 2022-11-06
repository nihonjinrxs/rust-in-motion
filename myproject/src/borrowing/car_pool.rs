#[derive(Debug)]
pub struct CarPool {
    pub passengers: Vec<String>,
}

impl CarPool {
    /// Add the named passenger to the car pool
    pub fn pick_up(&mut self, name: String) {
        self.passengers.push(name);
    }
}
