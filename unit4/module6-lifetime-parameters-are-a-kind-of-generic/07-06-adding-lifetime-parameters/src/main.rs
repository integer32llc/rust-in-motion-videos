pub struct Stemmer {
    pub suffix: String,
}

impl Stemmer {
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
