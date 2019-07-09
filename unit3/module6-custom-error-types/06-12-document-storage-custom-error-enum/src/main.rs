use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

pub enum DocumentServiceError {
    RateLimitExceeded,
    Io(io::Error),
}

pub fn create_document(filename: &str) -> Result<File, Box<Error>> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err("You have exceeded the allowed number of documents per minute".into());
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}

fn main() {}
