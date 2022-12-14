use std::{
    fs::{File, OpenOptions},
    io,
    result,
};
use failure::{Backtrace, Fail};

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

#[derive(Debug, Fail)]
pub enum DocumentServiceError {
    #[fail(display = "You have exceeded the allowed number of documents per minute.")]
    RateLimitExceeded(Backtrace),
    #[fail(display = "I/O error: {}", _0)]
    Io(io::Error, Backtrace),
}

impl From<io::Error> for DocumentServiceError {
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other, Backtrace::new())
    }
}

// Ensure all results returned from module use the same error enum type
pub type Result<T> = result::Result<T, DocumentServiceError>;

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded(Backtrace::new()));
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?; // From trait converts all io::Error to DocumentServiceError::Io(io::Error)

    Ok(file)
}

pub fn list_documents() -> Result<Vec<File>> {
    unimplemented!()
}

pub fn archive_document(filename: &str) -> Result<()> {
    unimplemented!()
}

pub fn create_project(project_name: &str) -> Result<()> {
    create_document(&format!("{}-draft1", project_name))?;
    create_document(&format!("{}-draft2", project_name))?;
    create_document(&format!("{}-revision1", project_name))?;
    create_document(&format!("{}-revision2", project_name))?;
    Ok(())
}

pub fn run() {
    match create_project("my-project") {
        Ok(()) => println!("Project created successfully"),
        Err(e) => {
            println!("Project creation failed: {}", e);
            if let Some(backtrace) = e.backtrace() {
                if !backtrace.to_string().trim().is_empty() {
                    println!("backtrace: {:?}", backtrace);
                }
            }
        },
    }
}
