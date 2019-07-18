// The lifetime elision rules mean the compiler interprets this signature as if we had written:
// fn ex<'a>(color_name: &'a str, saturation: u8) -> &'a Color
fn ex(color_name: &str, saturation: u8) -> &Color {
    unimplemented!();
}

struct Color;

fn main() {}
