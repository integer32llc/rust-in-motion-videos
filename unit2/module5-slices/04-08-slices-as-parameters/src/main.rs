fn main() {
    let a = [1, 2, 3];
    let v = vec![4, 5, 6];
    let v_slice = &v[..];

    only_reference_to_array(&a);
    only_reference_to_vector(&v);
    reference_to_either_array_or_vector(&a[..]);
    reference_to_either_array_or_vector(&v[..]);
    reference_to_either_array_or_vector(&v_slice[0..1]);
}

fn only_reference_to_array(param: &[i32; 3]) {
    println!("this is an array: {:?}", param);
}

fn only_reference_to_vector(param: &Vec<i32>) {
    println!("this is a vector: {:?}", param);
}

fn reference_to_either_array_or_vector(param: &[i32]) {
    println!("this is a slice: {:?}", param);
}