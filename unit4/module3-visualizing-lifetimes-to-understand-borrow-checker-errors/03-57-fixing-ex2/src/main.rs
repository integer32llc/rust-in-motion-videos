fn main() {
    let list = return_list();
    let first_two = &list[0..2];
    println!("First two are {:?}", first_two);
}

fn return_list() -> Vec<i32> {
    vec![100, 34, 72, 55]
}
