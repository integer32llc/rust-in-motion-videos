fn main() {

    let die1 = 1;
    let die2 = 5;

    match (die1, die2) {
        (1, 1) => println!("Snake eyes! Go back to the beginning."),
        (5, _) | (_, 5) => {
            println!("You rolled at least one 5!");
            println!("Move and then roll again!");
        },
        _ => println!("Move your piece!"),
    }
}
