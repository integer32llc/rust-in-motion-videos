fn main() {
    let option_value = Some(25);
    if option_value.is_some() {
        let inner = option_value.unwrap();
        println!("inner = {}", inner);
    }
}
