extern crate http; // 0.1.10

use http::header::HeaderName;

fn main() {
    let _h = HeaderName::from_static("no-special-characters");
}
