fn save_status(text: &str) -> Result<i64, &'static str> {
    if text.len() > 200 {
        return Err("status text is too long");
    }
}

fn main() {
    println!("Hello, world!");
}
