use std::io;

fn main() {
    loop {
        println!("What's the secret word?");
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");

        if word.trim() == "rust" {
            break;
        }
    }

    println!("You know the secret word! Please proceed!");
}
