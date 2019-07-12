extern crate rand;

fn simulate_game<'a>(home: &'a str, away: &'a str) -> &'a str {
    if rand::random() {
        home
    } else {
        away
    }
}

fn main() {
    let team1 = String::from("Panthers");
    let winner = {
        let team2 = String::from("Yellow Jackets");

        simulate_game(&team1, &team2)
    };
    println!("{} won", winner);
}
