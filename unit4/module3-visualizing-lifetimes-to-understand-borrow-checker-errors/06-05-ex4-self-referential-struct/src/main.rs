struct ListAndRef {
    list: Vec<i32>,
    first_two: &[i32],
}

fn return_list_and_first_two() -> ListAndRef {
    let list_to_use = vec![100, 34, 72, 55];

    ListAndRef {
        list: list_to_use,
        first_two: &list_to_use[0..2],
    }
}

fn main() {}
