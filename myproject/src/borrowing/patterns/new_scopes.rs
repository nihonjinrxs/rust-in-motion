// Basic idea: indicate to borrow checker where a borrow should end

pub fn run() {
    let mut list = vec![1, 2, 3];
    
    { // Create a new inner scope here to ensure variables go out of scope before end
        let list_first = list.first(); // immutable reference 1
        let list_last = list.last(); // immutable reference 2
        
        // This will fail after NLL, because we can't take a mutable reference while we have immutable ones
        // let list_first_mut = list.first_mut().expect("list was empty");
        // *list_first_mut += 1;

        println!(
            "The first element is {:?} and the last is {:?}",
            list_first,
            list_last,
        );
    } // immutable references 1 and 2 out of scope now

    // No longer errors, but used to without inner scope; Rust borrow checker better now.
    *list.first_mut().expect("list was empty") += 1;
}
