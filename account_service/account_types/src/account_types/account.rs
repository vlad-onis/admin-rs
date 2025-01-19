use crate::account_types::common::{Email, Password, Username};

#[derive(Debug, Clone)]
pub struct Account {
    pub username: Username,
    pub password: Password,
    pub email: Email,
}
