fn say(s: String) {
    println!("I say, {}!", s);
}

fn main() {
    let a = String::from("hello");
    say(a);
    println!("Using a again: {}", a);
}
