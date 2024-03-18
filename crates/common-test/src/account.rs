use std::ops::Deref;

use crate::prelude::*;

pub struct AccountHeader {
    /// The account address.
    pub pubkey: Pubkey,
    /// Lamports in the account.
    pub lamports: u64,
    /// Account data.
    pub data: Vec<u8>,
    /// The program that owns the account. If executable, the program that loads this account.
    pub owner: Pubkey,
    /// Whether the account's data contains a loaded program (and is now read-only).
    pub executable: bool,
    /// The epoch at which the account will next owe rent.
    pub rent_epoch: Epoch,
}

pub struct AccountWithHeader<T> {
    pub header: AccountHeader,
    account: T,
}

impl<T> AccountWithHeader<T> {
    pub fn new<F>(pubkey: Pubkey, account: Account, f: F) -> Result<AccountWithHeader<T>>
    where
        F: FnOnce(&[u8]) -> Result<T>,
    {
        let Account { lamports, data, owner, executable, rent_epoch } = account;

        let account = f(&data)?;
        let header = AccountHeader { pubkey, lamports, data, owner, executable, rent_epoch };

        Ok(AccountWithHeader { header, account })
    }

    pub fn into_inner(self) -> T {
        self.account
    }

    pub fn into_parts(self) -> (AccountHeader, T) {
        (self.header, self.account)
    }

    pub async fn get_rent(&self, context: &mut ProgramTestContext) -> Result<u64> {
        context.get_balance_for_rent(self.header.data.len()).await
    }
}

impl<T> AsRef<AccountHeader> for AccountWithHeader<T> {
    fn as_ref(&self) -> &AccountHeader {
        &self.header
    }
}

impl<T> Deref for AccountWithHeader<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.account
    }
}
