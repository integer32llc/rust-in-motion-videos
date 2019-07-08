fn add_five(n: &str) -> i32 {
    let num: i32 = n.parse()?;
    num + 5
}

fn main() {
    println!("Hello, world!");
}
