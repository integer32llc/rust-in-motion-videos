fn main() {
    let list = vec![100, 34, 72, 55];
    let first_two = &list[0..2];
    println!("first two are {:?}", first_two);
    println!("list is {:?}", list);
    println!("again, first two: {:?}", first_two);
}
