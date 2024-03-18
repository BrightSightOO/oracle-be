use solana_sdk::sysvar::Sysvar;
use spl_associated_token_account::get_associated_token_address;

use crate::prelude::*;

#[async_trait]
pub trait ContextExt {
    async fn get_account_optional(&mut self, pubkey: Pubkey) -> Result<Option<Account>>;

    async fn get_account(&mut self, pubkey: Pubkey) -> Result<Account> {
        self.get_account_optional(pubkey).await?.ok_or_else(|| eyre!("account not found"))
    }

    async fn get_sysvar<T: Sysvar>(&mut self) -> Result<T> {
        let name = std::any::type_name::<T>();

        let account = self
            .get_account_optional(T::id())
            .await?
            .ok_or_else(|| eyre!("sysvar {name} not found"))?;

        bincode::deserialize(&account.data)
            .wrap_err_with(|| eyre!("failed to deserialize sysvar {name}"))
    }

    async fn get_balance_for_rent(&mut self, space: usize) -> Result<u64> {
        let rent = self.get_sysvar::<Rent>().await?;
        Ok(rent.minimum_balance(space))
    }

    async fn get_account_rent(&mut self, pubkey: Pubkey) -> Result<u64> {
        let account = self.get_account(pubkey).await?;
        let rent = self.get_sysvar::<Rent>().await?;
        Ok(rent.minimum_balance(account.data.len()))
    }

    async fn get_packed_account<T: Pack>(
        &mut self,
        pubkey: Pubkey,
    ) -> Result<AccountWithHeader<T>> {
        let name = std::any::type_name::<T>();
        let account = self.get_account(pubkey).await?;

        AccountWithHeader::new(pubkey, account, |data| {
            T::unpack_from_slice(data)
                .wrap_err_with(|| format!("failed to deserialize {name} account"))
        })
    }

    async fn get_borsh_account<T: BorshDeserialize>(
        &mut self,
        pubkey: Pubkey,
    ) -> Result<AccountWithHeader<T>> {
        let name = std::any::type_name::<T>();
        let account = self.get_account(pubkey).await?;

        AccountWithHeader::new(pubkey, account, |data| {
            let mut data = data;

            T::deserialize(&mut data)
                .wrap_err_with(|| format!("failed to deserialize {name} account"))
        })
    }

    async fn airdrop(&mut self, pubkey: Pubkey, lamports: u64) -> Result<()>;

    async fn create_mint(&mut self, decimals: u8) -> Result<Pubkey>;

    async fn create_token_account(&mut self, mint: Pubkey, owner: Pubkey) -> Result<Pubkey>;

    async fn create_associated_token_account(
        &mut self,
        mint: Pubkey,
        owner: Pubkey,
    ) -> Result<Pubkey>;

    async fn mint_tokens(&mut self, account: Pubkey, amount: u64) -> Result<()>;
}

#[async_trait]
impl ContextExt for ProgramTestContext {
    async fn get_account_optional(&mut self, pubkey: Pubkey) -> Result<Option<Account>> {
        self.banks_client.get_account(pubkey).await.wrap_err("failed to get account")
    }

    async fn airdrop(&mut self, pubkey: Pubkey, lamports: u64) -> Result<()> {
        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(&self.payer.pubkey(), &pubkey, lamports)],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            self.last_blockhash,
        );
        self.banks_client.process_transaction(tx).await?;

        Ok(())
    }

    async fn create_mint(&mut self, decimals: u8) -> Result<Pubkey> {
        let mint = Keypair::new();

        let rent = self.get_sysvar::<Rent>().await?;

        let tx = Transaction::new_signed_with_payer(
            &[
                system_instruction::create_account(
                    &self.payer.pubkey(),
                    &mint.pubkey(),
                    rent.minimum_balance(Mint::LEN),
                    Mint::LEN as u64,
                    &spl_token::ID,
                ),
                spl_token::instruction::initialize_mint2(
                    &spl_token::ID,
                    &mint.pubkey(),
                    &self.payer.pubkey(),
                    Some(&self.payer.pubkey()),
                    decimals,
                )?,
            ],
            Some(&self.payer.pubkey()),
            &[&self.payer, &mint],
            self.last_blockhash,
        );

        self.banks_client.process_transaction(tx).await?;

        Ok(mint.pubkey())
    }

    async fn create_token_account(&mut self, mint: Pubkey, owner: Pubkey) -> Result<Pubkey> {
        let account = Keypair::new();

        let rent = self.banks_client.get_rent().await?;

        let tx = Transaction::new_signed_with_payer(
            &[
                system_instruction::create_account(
                    &self.payer.pubkey(),
                    &account.pubkey(),
                    rent.minimum_balance(TokenAccount::LEN),
                    TokenAccount::LEN as u64,
                    &spl_token::ID,
                ),
                spl_token::instruction::initialize_account3(
                    &spl_token::ID,
                    &account.pubkey(),
                    &mint,
                    &owner,
                )?,
            ],
            Some(&self.payer.pubkey()),
            &[&self.payer, &account],
            self.last_blockhash,
        );

        self.banks_client.process_transaction(tx).await?;

        Ok(account.pubkey())
    }

    async fn create_associated_token_account(
        &mut self,
        mint: Pubkey,
        wallet: Pubkey,
    ) -> Result<Pubkey> {
        let account = get_associated_token_address(&wallet, &mint);

        let tx = Transaction::new_signed_with_payer(
            &[spl_associated_token_account::instruction::create_associated_token_account_idempotent(
                &self.payer.pubkey(),
                &wallet,
                &mint,
                &spl_token::ID,
            )
            ],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            self.last_blockhash,
        );

        self.banks_client.process_transaction(tx).await?;

        Ok(account)
    }

    async fn mint_tokens(&mut self, account: Pubkey, amount: u64) -> Result<()> {
        let account = self.get_packed_account::<TokenAccount>(account).await?;
        let mint = self.get_packed_account::<Mint>(account.mint).await?;

        let mint_authority = mint.mint_authority.ok_or_else(|| eyre!("no mint authority"))?;

        ensure!(mint_authority == self.payer.pubkey(), "incorrect mint authority");

        let tx = Transaction::new_signed_with_payer(
            &[spl_token::instruction::mint_to_checked(
                &mint.header.owner,
                &mint.header.pubkey,
                &account.header.pubkey,
                &mint_authority,
                &[],
                amount,
                mint.decimals,
            )?],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            self.last_blockhash,
        );

        self.banks_client.process_transaction(tx).await?;

        Ok(())
    }
}
