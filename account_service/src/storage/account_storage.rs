use crate::storage::account_row::AccountRow;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

pub trait AccountStorage {
    async fn insert(account: AccountRow) -> Result<(), Error>;

    async fn get_account_by_username(username: String) -> Result<Option<AccountRow>, Error>;

    async fn delete(account: AccountRow) -> Result<(), Error>;

    async fn upsert(account: AccountRow) -> Result<AccountRow, Error>;
}
