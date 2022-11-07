use std::error::Error;
use std::env;

fn num_threads() -> Result<usize, Box<dyn Error>> {
    let s = env::var("NUM_THREADS")?; // Err(env::VarError)
    let n: usize = s.parse()?; // Err(num::ParseIntError)
    Ok(n + 1)
}

// Simplest approach to typing multiple errors from a function in a Result type
// Box<Error> == Box<dyn Error> is a trait object
// - Pointer (Box)
// - Trait (std::error::Error)
// Error requires Debug and Display for printing to user
// Downside is that you can't later inspect the error type
// if you need that, use a custom error type

fn run_application() -> Result<(), Box<dyn Error>> {
    let num = num_threads()?;
    println!("the number of threads is {}", num);
    // Rest of the program's functionality
    Ok(())
}

pub fn run() {
    if let Err(e) = run_application() {
        panic!("An error happened {}", e);
    }
}
