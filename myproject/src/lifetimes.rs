pub mod references_in_hashmap;

pub fn run() {
    returning_ref_from_inner_scope_1();
    returning_ref_from_inner_scope_2();
    returning_ref_from_fn();
    referencing_moved_value();
    // list_and_first_two();

    references_in_hashmap::run();
}

// fn returning_ref_from_inner_scope() {
//     let first_two = {
//         let list = vec![100, 34, 72, 55];
//         // This reference is being returned outside, so its lifetime extends
//         // beyond the end of the block it's created in
//         &list[0..2]
//     };
    
//     println!("First two are {:?}", first_two);
// }

fn returning_ref_from_inner_scope_1() {
    // everything in inner scope
    {
        let list = vec![100, 34, 72, 55];
        let first_two = &list[0..2];

        println!("First two are {:?}", first_two);
    };
    
}

fn returning_ref_from_inner_scope_2() {
    // creation outside inner scope
    let list = vec![100, 34, 72, 55];
    let first_two = {
        &list[0..2]
    };

    println!("First two are {:?}", first_two);
}


// fn returning_ref_from_fn() {
//     let first_two = return_first_two();
//     println!("First two are {:?}", first_two);
// }
// fn return_first_two() -> &[i32] {
//     let list = vec![100, 34, 72, 55];
//     &list[0..2]
// }

// Move reference out of function since not really needed
fn returning_ref_from_fn() {
    let list = return_list();
    let first_two = &list[0..2];
    println!("First two are {:?}", first_two);
}

fn return_list() -> Vec<i32> {
    vec![100, 34, 72, 55]
}


// fn referencing_moved_value() {
//     let list_a = vec![100, 34, 72, 55];
//     let list_b = list_a;
//     let first_two = &list_a[0..2]; // borrow of moved value error happens here
//     println!("First two are {:?}", first_two);
// }

fn referencing_moved_value() {
    let list_a = vec![100, 34, 72, 55];
    let first_two = &list_a[0..2];
    println!("First two are {:?}", first_two);
    let list_b = list_a;
}


// Self-referential struct example
// struct ListAndRef {
//     list: Vec<i32>,
//     first_two: &[i32],
// }

// fn list_and_first_two() -> ListAndRef {
//     let list_to_use = vec![100, 34, 72, 55];

//     ListAndRef {
//         list: list_to_use,
//         first_two: &list_to_use[0..2]
//     }
// }

// If needed, check out rental and owning-ref crates that enable self-referential structs
