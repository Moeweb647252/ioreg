use thiserror::Error;

pub type Result<T> = std::result::Result<T, IORegError>;

#[derive(Error, Debug)]
pub enum IORegError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Plist parsing error: {0}")]
    PlistParsing(#[from] plist::Error),
    #[error("Plist value error: {0}")]
    PlistValue(&'static str),
}
