pub mod bucket;
pub mod car_pool;
pub mod patterns {
    pub mod new_scopes;
    pub mod temporary_variables;
    pub mod entry_api;
    pub mod splitting_up_structs;
}
pub mod socket_ownership;
pub mod drop_trait;

use bucket::{Bucket, pour};
use car_pool::{CarPool};
use patterns::{new_scopes, temporary_variables, entry_api, splitting_up_structs};

pub fn run() {
    // Basic ownership
    {
        let a = String::from("hello");
        println!("String a = {}", a);

        // errors on next println because ownership of a is transferred to s inside say
        // say(a);
        // does not error, because s gets a copy of a
        say(a.clone());

        println!("Printing after calling say(): {}", a);
    }

    // Ownership exercise and borrowing
    {
        // Exercise (ownership): pluralize a string
        let book = String::from("book");
        // println!("I have one {}, you have two {}", book, pluralize(book.clone()));
        // Rewritten using borrowing
        println!("I have one {}, you have two {}", book, pluralize(&book));
    }

    // Slices
    {
        let v = vec![10, 20, 30];
        // Compiles, but errors at runtime due to index out of range
        // let v_slice = &v[..9];
        // println!("v_slice is {:?}", v_slice);

        let s = "🌮🐱";
        // Compiles, but errors at runtime because slice index not at valid Unicode boundary
        // let s_slice = &s[0..1];
        // println!("s_slice is: {:?}", s_slice);

        let a = [1, 2, 3];
        let v_slice = &v[..];

        only_reference_to_array(&a);
        only_reference_to_vector(&v);
        // Actually don't need [..] range on these two due to Deref trait, see docs
        reference_to_either_array_or_vector(&a[..]);
        reference_to_either_array_or_vector(&v[..]);
        reference_to_either_array_or_vector(&v_slice[0..1]);

        let greeting = String::from("hi");
        let string_literal = "hello";

        either_string_or_literal(&s);
        either_string_or_literal(&greeting);
        either_string_or_literal(&string_literal);
    }

    // Borrowing mutable references
    {
        let mut bucket1 = Bucket { liters: 20 };
        let mut bucket2 = Bucket { liters: 10 };
        pour(&mut bucket1, &mut bucket2, 3);
        println!("Bucket 1: {:?}", bucket1);
        println!("Bucket 2: {:?}", bucket2);

        let mut monday_car_pool = CarPool {
            passengers: vec![],
        };
        monday_car_pool.pick_up(String::from("Jake"));
        println!("Car pool state: {:?}", monday_car_pool);
        monday_car_pool.pick_up(String::from("Carol"));
        println!("Car pool state: {:?}", monday_car_pool);

        // Example of non-compiling code: attempt to borrow mutable while immutable borrow exists
        // let mut list = vec![1, 2, 3];
        // for i in &list{ // immutable borrow here
        //     println!("i is {}", i);
        //     list.push(i + 1); // attempt to borrow mutable reference here
        // }
    }

    // Borrowing code patterns
    {
        new_scopes::run();
        temporary_variables::run();
        entry_api::run();
        splitting_up_structs::run();

        // Note: non-lexical lifetimes NLL
        // new scopes, temporary variables, entry_api examples fixed by NLL
        // entry_api still a better solution and more efficient too
    }

    // Ownership of other resources
    {
        socket_ownership::run();

        // Notes:
        // - Mutex<T> locks automatically released when out of scope
        // - Rc<T> (reference counter) automatically decrements counter when owner
        //   goes out of scope, automatically destroys Rc instance when no more owners
        // - Files automatically closed when file reference is out of scope

        drop_trait::run();
    }
}

fn say(s: String) {
    println!("I say, {}!", s);
}

// Exercise (ownership): pluralize a string
// fn pluralize(singular: String) -> String {
//     singular + "s"
// }
// Rewritten using borrowing
fn pluralize(singular: &str) -> String {
    singular.to_owned() + "s"
}

// Slice references
fn only_reference_to_array(param: &[i32; 3]) {
    println!("this is an array: {:?}", param);
}

fn only_reference_to_vector(param: &Vec<i32>) {
    println!("this is a vector: {:?}", param);
}

fn reference_to_either_array_or_vector(param: &[i32]) {
    println!("this is a slice: {:?}", param);
}

fn either_string_or_literal(param: &str) {
    println!("this is a string slice: {:?}", param);
}
