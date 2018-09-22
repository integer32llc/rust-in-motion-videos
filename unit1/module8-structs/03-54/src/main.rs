struct Triangle(u32, u32, u32);

fn is_equilateral(triangle: Triangle) -> bool {
    triangle.0 == triangle.1 && triangle.1 == triangle.2
}

fn main() {
    let triangle1 = Triangle(3, 4, 5);
    is_equilateral(triangle1);

    let nums = (5, 5, 5);
    is_equilateral(nums);
}
