fn add_five_to_last(n: &[i32]) -> Option<i32> {
    let num = n.last()?;
    Some(num + 5)
}

fn main() {
    let list = vec![1, 2, 3];
    println!("add_five_to_last on list: {:?}", add_five_to_last(&list));
    let empty_list = vec![];
    println!("add_five_to_last on empty list: {:?}", add_five_to_last(&empty_list));
}
