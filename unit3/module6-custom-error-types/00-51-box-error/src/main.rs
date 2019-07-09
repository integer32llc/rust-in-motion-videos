use std::error::Error;
use std::env;

fn num_threads() -> Result<usize, Box<Error>> {
    let s = env::var("NUM_THREADS")?;
    let n: usize = s.parse()?;
    Ok(n + 1)
}

fn main() {}
