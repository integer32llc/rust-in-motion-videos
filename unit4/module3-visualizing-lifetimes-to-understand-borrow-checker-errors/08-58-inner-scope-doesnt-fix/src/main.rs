use std::io;
use std::collections::HashMap;

fn main() {
    println!("Please enter some text to get word counts");
    let mut counts = HashMap::new();
    {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("problem reading input");

        for word in input.split_whitespace() {
            let count = counts.entry(word).or_insert(0);
            *count += 1;
        }
    }
    println!("counts = {:?}", counts);
}
