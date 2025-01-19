use account_types::account_types::account::Account;

pub struct AccountRow {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl From<Account> for AccountRow {
    fn from(value: Account) -> Self {
        AccountRow {
            username: String::from(value.username),
            password: String::from(value.password),
            email: String::from(value.email),
        }
    }
}
