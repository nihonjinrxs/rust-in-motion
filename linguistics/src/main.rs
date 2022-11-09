pub struct Stemmer {
    pub suffix: String,
}

impl Stemmer {
    // "Lifetime of the return type of the stem method is related only to the
    //  word parameter, not to the Stemmer instance"
    pub fn stem<'a>(&self, word: &'a str) -> &'a str {
        if word.ends_with(&self.suffix) {
            let index = word
                .rfind(&self.suffix)
                .expect("Should be found because ends_with returned true");
            &word[0..index]
        } else {
            word
        }
    }
}

fn main() {
    let word = String::from("credited");
    let word_stem = {
        let stemmer = Stemmer {
            suffix: String::from("ed"),
        };
        stemmer.stem(&word)
    };
    println!("The stem of {} is {}", word, word_stem);
}

/*
 * Notes on solving lifetime parameter errors:
 * - Let the compiler help! There are often helpful suggestions in the compiler errors.
 * - Analyze generic lifetimes and specify how you intend they be related
 * - Analyze concrete lifetimes where using the definitions
 */
