fn main() {
    let list = vec![100, 34, 72, 55];
    let first_two = return_first_two(&list);
    println!("first two are {:?}", first_two);
    println!("list is {:?}", list);
    println!("again, first two are {:?}", first_two);
}

fn return_first_two(borrowed_list: &[i32]) -> &[i32] {
    &borrowed_list[0..2]
}