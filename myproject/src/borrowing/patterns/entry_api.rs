// Basic idea: Updating or inserting into a data structure in a cleaner/managed way

use std::collections::HashMap;

pub fn run() {
    let text = "hello world hello";

    let mut freqs = HashMap::new();

    for word in text.split_whitespace() {
        // Used to error due to mutable borrow inside mutable borrow
        // match freqs.get_mut(word) { // mutable borrow 1
        //     Some(value) => *value += 1,
        //     None => { freqs.insert(word, 1); }, // mutable borrow 2
        // }

        // Fix still easier to write
        *freqs.entry(word).or_insert(0) += 1;
    }

    println!("Word frequencies: {:#?}", freqs);
}
