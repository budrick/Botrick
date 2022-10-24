use thiserror::Error;

/// SporkerError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum SporkerError {
    /// Represents a failure doing anything with rusqlite
    #[error("Database error error")]
    DatabaseError { source: rusqlite::Error },

    #[error("Path error")]
    PathError,

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
