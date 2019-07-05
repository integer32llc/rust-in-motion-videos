fn main() {
    let s = String::from("hello");

    let s_slice = &s[2..4];
    println!("The slice from 2 up to 4 is {}", s_slice);

    let s_slice = &s[0..4];
    println!("The slice from 0 up to 4 is {}", s_slice);

    let s_slice = &s[..4];
    println!("The slice without a starting index up to 4 is {}", s_slice);

    let s_slice = &s[2..5];
    println!("The slice from 2 up to 5 is {}", s_slice);

    let s_slice = &s[2..];
    println!("The slice from 2 without an ending index is {}", s_slice);

    let s_slice = &s[..];
    println!("The slice of the entire string is {}", s_slice);
}
