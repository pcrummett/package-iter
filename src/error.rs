use std::{error::Error as StdError, fmt, path::PathBuf};

/// `Result<T>` provides a simplified result type with a common error type
pub type Result<T> = std::result::Result<T, Error>;

// An error indicating that something went wrong with an arch linux operation
#[derive(Debug)]
pub enum Error {
    /// An error indicating that iterator construction failed.
    DatabaseIteration(PathBuf),

    /// An error indicating that the given database failed to load.
    DatabaseLoad(PathBuf),

    /// An error indicating that the given database was not found.
    DatabaseNotFound(String),

    /// An error indicating that the given package was not found.
    PackageNotFound(String),

    /// An error occurred while parsing package integers .
    PackageParseSize,

    /// An error indicating that a required package property is missing.
    PackagePropertyMissing(String),

    /// An error occurred during package utf8 conversion after extraction.
    PackageUtf8Conversion(String),
}
impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DatabaseIteration(ref path) => write!(
                f,
                "failed to construct database iterator: {}",
                path.display()
            ),
            Error::DatabaseLoad(ref path) => {
                write!(f, "failed to load database: {}", path.display())
            }
            Error::DatabaseNotFound(ref db) => write!(f, "failed to find database: {}", db),
            Error::PackageNotFound(ref pkg) => write!(f, "failed to find package: {}", pkg),
            Error::PackageParseSize => {
                write!(f, "package parse failure while parsing integers")
            }
            Error::PackagePropertyMissing(ref prop) => {
                write!(f, "package property missing: {}", prop)
            }
            Error::PackageUtf8Conversion(ref pkg) => write!(
                f,
                "package utf8 conversion failed after extraction: {}",
                pkg
            ),
        }
    }
}
