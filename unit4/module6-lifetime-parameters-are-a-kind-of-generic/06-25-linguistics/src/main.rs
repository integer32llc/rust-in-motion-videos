pub struct Stemmer {
    pub suffix: String,
}

impl Stemmer {
    pub fn stem(&self, word: &str) -> &str {
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
    println!("Hello, world!");
}
