#[derive(Debug, Clone)]
pub struct Username(String);

impl From<Username> for String {
    fn from(value: Username) -> Self {
        value.0
    }
}

#[derive(Debug, Clone)]
pub struct Password(String);

impl From<Password> for String {
    fn from(value: Password) -> Self {
        value.0
    }
}

#[derive(Debug, Clone)]
pub struct Email(String);

impl From<Email> for String {
    fn from(value: Email) -> Self {
        value.0
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    pub fn inner_username() {
        let username = Username("vladonis".to_string());
        assert_eq!(String::from(username), "vladonis".to_string());
    }

    #[test]
    pub fn inner_password() {
        let password = Password("vladonis123".to_string());
        assert_eq!(String::from(password), "vladonis123".to_string());
    }

    #[test]
    pub fn inner_email() {
        let email = Email("vladonis@vladonis.com".to_string());
        assert_eq!(String::from(email), "vladonis@vladonis.com".to_string());
    }
}
