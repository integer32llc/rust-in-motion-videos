extern crate rand;

fn simulate_game(home: &str, away: &str) -> &str {
    if rand::random() {
        home
    } else {
        away
    }
}

fn main() {
    println!("Hello, world!");
}
