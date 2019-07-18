// The lifetime elision rules mean the compiler interprets this signature as if we had written:
// fn ex<'a, 'b>(amt: i32, name: &'a str, user: 'b User) -> &str
fn ex(amt: i32, name: &str, user: &User) -> &str {
    unimplemented!()
}

struct User {
    username: String,
}

fn main() {}
