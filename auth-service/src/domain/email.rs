use std::hash::Hash;
use validator::validate_email;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

#[derive(Debug, PartialEq)]
pub enum EmailParseError {
    EmptyEmail,
    InvalidFormat,
}

impl Email {
    pub fn parse(email: String) -> Result<Email, EmailParseError> {
        if email.is_empty() {
            return Err(EmailParseError::EmptyEmail);
        }
        
        if !validate_email(&email) {
            return Err(EmailParseError::InvalidFormat);
        }
        
        Ok(Email(email))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    #[derive(Debug, Clone)]
    struct ValidEmail(String);

    impl Arbitrary for ValidEmail {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[test]
    fn test_empty_email_is_rejected() {
        let email = "".to_string();
        let result = Email::parse(email);
        assert_eq!(result, Err(EmailParseError::EmptyEmail));
    }

    #[test]
    fn test_email_without_at_symbol_is_rejected() {
        let email = "invalid-email".to_string();
        let result = Email::parse(email);
        assert_eq!(result, Err(EmailParseError::InvalidFormat));
    }

    #[test]
    fn test_valid_email_is_accepted() {
        let email = "test@example.com".to_string();
        let result = Email::parse(email);
        assert!(result.is_ok());
    }

    #[test]
    fn test_as_ref() {
        let email = Email::parse("test@example.com".to_string()).unwrap();
        assert_eq!(email.as_ref(), "test@example.com");
    }

    #[quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmail) -> bool {
        Email::parse(valid_email.0).is_ok()
    }

    #[test]
    fn test_email_missing_domain() {
        let email = "user@".to_string();
        let result = Email::parse(email);
        assert_eq!(result, Err(EmailParseError::InvalidFormat));
    }

    #[test]
    fn test_email_missing_user() {
        let email = "@example.com".to_string();
        let result = Email::parse(email);
        assert_eq!(result, Err(EmailParseError::InvalidFormat));
    }
}

