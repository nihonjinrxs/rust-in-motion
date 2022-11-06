pub fn run() {
    let a = String::from("hello");
    println!("String a = {}", a);

    // errors on next println because ownership of a is transferred to s inside say
    // say(a);
    // does not error, because s gets a copy of a
    say(a.clone());

    println!("Printing after calling say(): {}", a);

    // Exercise (ownership): pluralize a string
    let book = String::from("book");
    println!("I have one {}, you have two {}", book, pluralize(book.clone()));
}

fn say(s: String) {
    println!("I say, {}!", s);
}

// Exercise (ownership): pluralize a string
fn pluralize(singular: String) -> String {
    singular + "s"
}
