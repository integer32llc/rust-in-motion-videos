fn main() {
    let is_confirmed = true;
    let is_active = false;

    match (is_confirmed, is_active) {
        (true, true) => println!("Your account is in good standing."),
        (false, true) => println!("You need to confirm your account!"),
        (false, false) => println!("This account will be deactivated."),
        _ => {},
    }
}