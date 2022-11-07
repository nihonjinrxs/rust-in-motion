use std::io;
use std::collections::HashMap;

// broken
pub fn run() {
    // broken();
    single_line_input_fix();
    // multiple_line_input_fix();
}

// fn broken() {
//     println!("Please enter some text to get word counts");
//     let mut counts = HashMap::new();
//     {
//         let mut input = String::new();

//         io::stdin().read_line(&mut input)
//             .expect("problem reading input");
        
//         for word in input.split_whitespace() {
//             let count = counts.entry(word).or_insert(0);
//             *count += 1;
//         }
//     } // input cleaned up at end of inner scope (without inner scope, technically same, but Rust borrow checker can deal with it)

//     println!("counts = {:?}", counts);
// }

fn single_line_input_fix() {
    println!("Please enter some text to get word counts");
    // swap so input declared first, cleaned up after counts (scope cleanup in reverse of allocation)
    let mut input = String::new();
    let mut counts = HashMap::new();

    io::stdin().read_line(&mut input)
        .expect("problem reading input");
    
    for word in input.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    println!("counts = {:?}", counts);
}

fn multiple_line_input_fix() {
    println!("Please enter some text to get word counts");
    let mut counts = HashMap::new();
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("problem reading input");
        
        for word in input.split_whitespace() {
            // copy each word to owned string when passing to entry
            let count = counts.entry(word.to_string()).or_insert(0);
            *count += 1;
        }

        println!("counts = {:?}", counts);
    }
}
