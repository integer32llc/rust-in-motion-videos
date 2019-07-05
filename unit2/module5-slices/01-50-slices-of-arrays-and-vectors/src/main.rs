fn main() {
    let a = [0.0, 3.14, -8.7928];
    let a_slice = &a[..1];
    println!("a_slice is {:?}", a_slice);

    let mut v = vec![10, 20, 30];
    v.push(40);
    let v_slice = &v[1..2];
    println!("v_slice is {:?}", v_slice);
}
