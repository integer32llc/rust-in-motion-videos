fn main() {
    let v = vec![10, 20, 30];
    let v_slice = &v[..9];
    println!("v_slice is: {:?}", v_slice);
}
