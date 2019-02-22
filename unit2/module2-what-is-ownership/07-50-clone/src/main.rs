fn say(s: String) {
    println!("I say, {}!", s);
}

fn main() {
    let a = String::from("hello");
    say(a.clone());
    println!("Using a again: {}", a);
}
