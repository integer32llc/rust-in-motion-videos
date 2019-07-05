#[derive(Debug)]
struct Bucket {
    liters: u32,
}

fn pour(source: &mut Bucket, target: &mut Bucket, amount: u32) {
    source.liters -= amount;
    target.liters += amount;
}

fn main() {
    let mut bucket1 = Bucket { liters: 20 };
    let mut bucket2 = Bucket { liters: 10 };

    pour(&mut bucket1, &mut bucket2, 3);

    println!("Bucket 1: {:?}", bucket1);
    println!("Bucket 2: {:?}", bucket2);
}
