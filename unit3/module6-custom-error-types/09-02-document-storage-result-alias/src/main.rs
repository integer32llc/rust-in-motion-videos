use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::fmt;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

#[derive(Debug)]
pub enum DocumentServiceError {
    RateLimitExceeded,
    Io(io::Error),
}

impl Error for DocumentServiceError {
    fn description(&self) -> &str {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => "rate limit exceeded",
            Io(_) => "I/O error",
        }
    }
}

impl fmt::Display for DocumentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => write!(
                f,
                "You have exceeded the allowed number of documents per minute."
            ),
            Io(ref io) => write!(f, "I/O error: {}", io),
        }
    }
}

impl From<io::Error> for DocumentServiceError {
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other)
    }
}

use std::result;

pub type Result<T> = result::Result<T, DocumentServiceError>;

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}

pub fn archive_document(filename: &str) -> Result<()> {
    unimplemented!();
}

pub fn list_documents() -> Result<Vec<File>> {
    unimplemented!();
}

fn main() {}
