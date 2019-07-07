fn main() {
    let nonempty_list = vec!['a', 'b', 'c'];
    println!("nonempty_list's last is: {:?}", nonempty_list.last());

    let empty_list: Vec<char> = vec![];
    println!("empty_list's last is: {:?}", empty_list.last());
}
