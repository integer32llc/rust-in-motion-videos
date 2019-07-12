extern crate rand;

fn simulate_game(home: &str, away: &str) -> &str {
    if rand::random() {
        home
    } else {
        away
    }
}

fn main() {
    let team1 = String::from("Panthers");
    {
        let team2 = String::from("Yellow Jackets");
        let winner = simulate_game(&team1, &team2);
        println!("{} vs {}: {} won", team1, team2, winner);
    }
    println!("Can still discuss {} here", team1);
}
