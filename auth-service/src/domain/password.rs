use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Password(String);

#[derive(Debug, PartialEq)]
pub enum PasswordParseError {
    EmptyPassword,
    TooShort,
}

impl Password {
    pub fn parse(password: String) -> Result<Password, PasswordParseError> {
        if password.is_empty() {
            return Err(PasswordParseError::EmptyPassword);
        }
        
        if password.len() < 8 {
            return Err(PasswordParseError::TooShort);
        }
        
        Ok(Password(password))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    #[derive(Debug, Clone)]
    struct ValidPassword(String);

    impl Arbitrary for ValidPassword {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let password = FakePassword(8..100).fake_with_rng(g);
            Self(password)
        }
    }

    #[test]
    fn test_empty_password_is_rejected() {
        let password = "".to_string();
        let result = Password::parse(password);
        assert_eq!(result, Err(PasswordParseError::EmptyPassword));
    }

    #[test]
    fn test_short_password_is_rejected() {
        let password = "short".to_string();
        let result = Password::parse(password);
        assert_eq!(result, Err(PasswordParseError::TooShort));
    }

    #[test]
    fn test_7_char_password_is_rejected() {
        let password = "1234567".to_string();
        let result = Password::parse(password);
        assert_eq!(result, Err(PasswordParseError::TooShort));
    }

    #[test]
    fn test_8_char_password_is_accepted() {
        let password = "12345678".to_string();
        let result = Password::parse(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_password_is_accepted() {
        let password = "password123".to_string();
        let result = Password::parse(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_as_ref() {
        let password = Password::parse("password123".to_string()).unwrap();
        assert_eq!(password.as_ref(), "password123");
    }

    #[quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidPassword) -> bool {
        Password::parse(valid_password.0).is_ok()
    }
}

