fn main() {
    let option_value = Some(25);
    if let Some(inner) = option_value {
        println!("inner = {}", inner);
    }
}
