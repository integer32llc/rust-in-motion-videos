fn main() {
    let first_two = {
        let list = vec![100, 34, 72, 55];
        &list[0..2]
    };

    println!("First two are {:?}", first_two);
}
