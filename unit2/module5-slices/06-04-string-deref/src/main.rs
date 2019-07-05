fn main() {
    let s = String::from("book");

    let pl = pluralize(&s);

    println!(
        "I have one {}, you have two {}",
        s,
        pl,
    );
}

fn pluralize(singular: &str) -> String {
    singular.to_owned() + "s"
}
