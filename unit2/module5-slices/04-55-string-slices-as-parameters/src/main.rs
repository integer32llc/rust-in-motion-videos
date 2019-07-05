fn main() {
    let s = String::from("hi");
    let string_literal = "hello";

    either_string_or_literal(&s);
    either_string_or_literal(&string_literal);
}

fn either_string_or_literal(param: &str) {
    println!("this is a string slice: {:?}", param);
}
