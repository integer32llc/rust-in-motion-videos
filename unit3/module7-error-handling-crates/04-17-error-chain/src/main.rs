#[macro_use]
extern crate error_chain;

use std::fs::{File, OpenOptions};

pub mod errors {
    error_chain! {
        errors {
            RateLimitExceeded {
                display("You have exceeded the allowed number of documents per minute.")
            }
        }
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

use errors::*;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        bail!(ErrorKind::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .chain_err(|| format!("could not open {}", filename))?;

    Ok(file)
}

fn create_project(project_name: &str) -> Result<()> {
    create_document(&format!("{}-draft1", project_name))?;
    create_document(&format!("{}-draft2", project_name))?;
    create_document(&format!("{}-revision1", project_name))?;
    create_document(&format!("{}-revision2", project_name))?;
    Ok(())
}

fn main() {
    match create_project("my-project") {
        Ok(()) => println!("Project created successfully"),
        Err(e) => {
            println!("Project creation failed: {}", e);
            for e in e.iter().skip(1) {
                println!("Caused by: {}", e);
            }
            if let Some(backtrace) = e.backtrace() {
                println!("backtrace: {:?}", backtrace);
            }
        }
    }
}
