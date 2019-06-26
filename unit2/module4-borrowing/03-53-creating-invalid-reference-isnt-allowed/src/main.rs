fn name() -> &str {
    let n = String::from("Carol");
    &n
}

fn main() {
    let my_name = name();
}
