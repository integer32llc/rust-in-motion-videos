use std::env;
use std::error::Error;

fn num_threads() -> Result<usize, Box<Error>> {
    let s = env::var("NUM_THREADS")?;
    let n: usize = s.parse()?;
    Ok(n + 1)
}

fn run_application() -> Result<(), Box<Error>> {
    let num = num_threads()?;
    println!("the number of threads is {}", num);
    // Rest of the program's functionality
    Ok(())
}

fn main() {
    if let Err(e) = run_application() {
        panic!("An error happened: {}", e);
    }
}
