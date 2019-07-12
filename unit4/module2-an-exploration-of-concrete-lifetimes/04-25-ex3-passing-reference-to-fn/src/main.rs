fn main() {
    let list = vec![100, 34, 72, 55];
    print_first_two(&list);
    println!("list is {:?}", list);
    print_first_two(&list);
}

fn print_first_two(borrowed_list: &[i32]) {
    let first_two = &borrowed_list[0..2];
    println!("first two are {:?}", first_two);
}