struct Noisy {
    id: i32,
}

// Drop trait lets you customize what happens when a variable goes out of scope
impl Drop for Noisy {
    fn drop(&mut self) {
        println!("Noisy number {} going out of scope!", self.id);
    }
}

pub fn run() {
    let _n1 = Noisy { id: 1 };
    let _n2 = Noisy { id: 2 };
    println!("End of Noisy numbers scope");
}
