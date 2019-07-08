fn add_five(n: &str) -> Result<i32, std::num::ParseIntError> {
    let num: i32 = n.parse()?;
    Ok(num + 5)
}

fn main() {
    println!("Hello, world!");
}
