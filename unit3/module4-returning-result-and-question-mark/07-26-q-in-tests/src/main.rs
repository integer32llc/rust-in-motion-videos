#[test]
fn parsing_works() -> Result<(), Box<dyn std::error::Error>> {
    let x: i32 = "3".parse()?;
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
