extern crate serde;
extern crate serde_json;

#[derive(Deserialize, Debug)]
struct Person {
    name: String,
}

pub fn run() {
    // Returns an Ok variant, so will succeed
    parse_person(r#"{ "name": "Margaret Hamilton" }"#);
    
    // Returns an Err variant, so will panic
    // parse_person(r#"{ "name": "Margaret Hamilton", }"#);
}

fn parse_person(json_string: &str) {
    let res = serde_json::from_str::<Person>(json_string);
    let person = get_person(res);
    println!("person's name = {:?}", person.name);
}

fn get_person(result: Result<Person, serde_json::Error>) -> Person {
    match result {
        Ok(inner) => inner,
        Err(error) => {
            if error.is_eof() { // not finished streaming data?
                // really, read more data then retry
                unimplemented!()
            } else {
                panic!("{:?}", error);
            }
        },
    }
}
