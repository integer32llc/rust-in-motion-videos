pub struct Stemmer {
    pub suffix: String,
}

impl Stemmer {
    pub fn stem(&self, word: &str) -> &str {
        if word.ends_with(&self.suffix) {
            let index = word
                .rfind(&self.suffix)
                .expect("found because ends_with returned true");
            &word[0..index]
        } else {
            word
        }
    }
}

#[test]
fn suffix_removal() {
    let stemmer = Stemmer { suffix: String::from("ed") };

    assert_eq!(stemmer.stem("jumped"), "jump");
    assert_eq!(stemmer.stem("jump"), "jump");

    assert_eq!(stemmer.stem("credited"), "credit");
    assert_eq!(stemmer.stem("credit"), "credit");
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
