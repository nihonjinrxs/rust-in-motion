use std::{
    error::Error,
    fs::{File, OpenOptions},
    io,
    result,
};
use quick_error::ResultExt;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 100;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

quick_error! {
    #[derive(Debug)]
    pub enum DocumentServiceError {
        RateLimitExceeded {
            display("You have exceeded the allowed number of documents per minute.")
        }
        Io(filename: String, cause: io::Error) {
            display("I/O error: {} for filename {}", cause, filename)
            context(filename: &'a str, cause: io::Error)
                -> (filename.to_string(), cause)
        }
    }
}

// Ensure all results returned from module use the same error enum type
pub type Result<T> = result::Result<T, DocumentServiceError>;

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .context(filename)?; // using the context function above on the custom error type

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
        Err(e) => println!("Project creation failed: {}", e),
    }
}
