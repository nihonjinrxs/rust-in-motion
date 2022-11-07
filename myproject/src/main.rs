pub mod syntax;         // section 1 code
pub mod borrowing;      // section 2 code
pub mod error_handling; // section 3 code
pub mod lifetimes;      // section 4 code

// For error_handling/json_results
#[macro_use]
extern crate serde_derive;

// For error_handling/using_quick_error
#[macro_use]
extern crate quick_error;

// For error_handling/using_failure
extern crate failure;
#[macro_use]
extern crate failure_derive;

fn main() {
    // syntax::run();
    // borrowing::run();
    // error_handling::run();
    lifetimes::run();
}
