use std::collections::HashMap;

struct Config {
    version: usize,
    settings: HashMap<String, String>,
}

impl Config {
    // The lifetime elision rules mean the compiler interprets this signature as if we had written:
    // fn get_value<'a, 'b>(&'a self, key: &'b str) -> &'a str {
    fn get_value(&self, key: &str) -> &str {
        self.settings.get(key).expect("key should exist")
    }
}

fn main() {}
