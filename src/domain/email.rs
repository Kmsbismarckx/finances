use crate::domain::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(raw: &str) -> Result<Self, DomainError> {
        let trimmed = raw.trim();
        let is_valid = trimmed.contains('@')
            && !trimmed.starts_with('@')
            && !trimmed.ends_with('@')
            && trimmed.split('@').count() == 2;

        if !is_valid {
            return Err(DomainError::InvalidEmail);
        }

        Ok(Email(trimmed.to_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_email() {
        assert!(Email::new("user@example.com").is_ok());
    }

    #[test]
    fn rejects_missing_at() {
        assert_eq!(Email::new("not-an-email"), Err(DomainError::InvalidEmail));
    }

    #[test]
    fn normalizes_case() {
        let sut = Email::new("User@Example.COM").unwrap();
        assert_eq!(sut.as_str(), "user@example.com");
    }
}
