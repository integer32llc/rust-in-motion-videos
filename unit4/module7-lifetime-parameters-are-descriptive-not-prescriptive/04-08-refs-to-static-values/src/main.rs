fn main() {
    let first_two = return_first_two();
    println!("First two are {:?}", first_two);
}

static LIST: [i32; 4] = [100, 34, 72, 55];

fn return_first_two() -> &'static [i32] {
    &LIST[0..2]
}