fn main() {
    let a = [1, 2, 3];
    let v = vec![4, 5, 6];
    let v_slice = &v[..];

    only_array(&a);
    only_vector(&v);
    either_array_or_vector(&a);
    either_array_or_vector(&v);
    either_array_or_vector(&v_slice[0..1]);
}

fn only_array(param: &[i32; 3]) {
    println!("this is an array: {:?}", param);
}

fn only_vector(param: &Vec<i32>) {
    println!("this is a vector: {:?}", param);
}

fn either_array_or_vector(param: &[i32]) {
    println!("this is a slice: {:?}", param);
}