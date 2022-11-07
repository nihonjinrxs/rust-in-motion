pub mod platform;
pub mod door;
pub mod json_results;
pub mod vec_last_option;
pub mod save_status;
pub mod box_error;
pub mod custom_error_types;
pub mod using_quick_error;
pub mod using_failure;

pub fn run() {
    // panic! macro
    {
        // panic!("Something is very wrong! Args: {}", "no")

        // No panic
        platform::run("windows");
        // Panics due to unknown platform
        // platform::run("oranges");

        // When to panic?
        // - Continuing would be incorrect or result in incorrect data
        // - No way for code to recover
        // - The problem must be fixed in code
        //   See http::header::HeaderName::from_static
        // - Failure is not ever expected to happen
    }

    // other panicking macros
    {
        // door::run();

        // the following macros also panic the program
        // - unreachable! - should be impossible to get to this spot in the code (see door::run() above)
        // - unimplemented! - code isn't yet written for this
        // - assert!, assert_eq!, assert_ne! - panic if a condition is not true (precondition checking, tests)
    }

    // Result and Option types, ? operator
    {
        json_results::run();
        vec_last_option::run();
        
        // => panic! database unavailable
        // save_status::save_status("some text").unwrap();
        
        // => panic! status text is too long
        // save_status::save_status(r#"Some really long text that goes on
        // for a long long time and keeps on going forever and ever and ever and ever 
        // and ever and ever and ever and ever and ever and ever and ever... 
        // "#).unwrap();
    }

    // Box<Error>
    {
        // To see this run without failing, use `NUM_THREADS=5 cargo run`
        // box_error::run();
    }

    // Custom error types & error crates
    {
        // Run first to create files...
        custom_error_types::run();
        // These will all error
        custom_error_types::run();
        using_quick_error::run();
        using_failure::run();

        // Note: error-chain library is now deprecated/archived
    }
}

/*
 * Notes on Result and Option methods:
 * - Testing variants: is_ok, is_err, is_some, is_none
 * - Converting Result <-> Option: ok, ok_or(err)
 * - Fallback values: unwrap_or(fallback), unwrap_or_default() using Default::default() trait
 * - Transform Ok or Some values only: map
 */
