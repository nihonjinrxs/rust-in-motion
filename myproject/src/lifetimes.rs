pub mod references_in_hashmap;
pub mod generics;

pub fn run() {
    returning_ref_from_inner_scope_1();
    returning_ref_from_inner_scope_2();
    returning_ref_from_fn();
    referencing_moved_value();
    // list_and_first_two();

    // references_in_hashmap::run();
    generics::run();
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

/*
 * Notes on lifetime parameters:
 * - 'static is for static values that live during the whole program,
 *   doesn't need to be specified in <>
 * - lifetime parameters don't make values live longer than they
 *   already do, they only describe to the compiler the relationship
 *   between values and their lifetimes
 * - when there's a lifetime error, ask how to rearrange code, not how
 *   to extend the borrow
 * 
 * Lifetime elision - default lifetimes that allow you to leave out lifetime parameters
 * Lifetime elision rules:
 * 1. In function parameters, each reference (&...) gets its own lifetime
 *    Example:
 *      fn ex(amt: i32, name: &str, user: &User) -> &str
 *    becomes
 *      fn ex<'a, 'b>(amt: i32, name: &'a str, user: &'b User) -> &str
 *    Note: In this case, this would not compile without lifetime rules, since the
 *          return type is not able to be automatically assigned a lifetime
 * 2. If there's only one lifetime in the function parameters, returned reference
 *    gets that lifetime
 *    Example:
 *       fn ex(color_name: &str, saturation: u8) -> &Color
 *    becomes
 *       fn ex<'a>(color_name: &'a str, saturation: u8) -> &'a Color
 * 3. If there is a &self or &mut self parameter (i.e., function is a method),
 *    returned reference gets that lifetime
 *    Example:
 *      struct Config {
 *        version: usize,
 *        settings: HashMap<String, String>,
 *      }
 *      impl Config {
 *        fn get_value(&self, key: &str) -> &str {}
 *      }
 *    becomes
 *      struct Config {
 *        version: usize,
 *        settings: HashMap<String, String>,
 *      }
 *      impl Config {
 *        fn get_value<'a, 'b>(&'a self, key: &'b str) -> &'a str {}
 *      }
 */
