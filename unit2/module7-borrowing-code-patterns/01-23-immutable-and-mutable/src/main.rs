fn main() {
    let mut list = vec![1, 2, 3];

    let list_first = list.first();
    let list_last = list.last();

    let list_first_mut = list.first_mut().expect("list was empty");
    *list_first_mut += 1;

    println!(
        "The first element is {:?} and the last is {:?}",
        list_first,
        list_last,
    );
}
