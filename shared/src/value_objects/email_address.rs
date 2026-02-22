use serde::{Deserialize, Serialize};

// RFC 5321 / RFC 5322
#[derive(Debug, thiserror::Error)]
pub  enum EmailError {
    #[error("The email address cannot be empty.")]
    EmptyEmailNotAllowed,
    #[error("The email address must contain an @ symbol.")]
    MissingAtSymbol,
    #[error("Email addresses cannot begin with @.")]
    InvalidLocalPart,
    #[error("The email requires a valid domain name after the @ symbol.")]
    InvalidDomainPart,
    #[error("Email cannot exceed 64 characters before an @ symbol.")]
    TooLongLocalPart(usize),
    #[error("Email cannot exceed 255 characters after an @ symbol.")]
    TooLongDomainPart(usize),
    #[error("Email cannot exceed 254 characters.")]
    TooLong(usize),
    #[error("Email must be at least 1 characters, got {0}")]
    TooShort(usize),
    #[error("Email cannot have consecutiva dots")]
    ConsecutiveDots,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddress;

impl  {
    
}