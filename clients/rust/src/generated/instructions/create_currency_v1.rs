//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::Bounds;
use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct CreateCurrencyV1 {
    /// Config
    pub config: solana_program::pubkey::Pubkey,
    /// Currency
    pub currency: solana_program::pubkey::Pubkey,
    /// Mint
    pub mint: solana_program::pubkey::Pubkey,
    /// Oracle authority
    pub authority: solana_program::pubkey::Pubkey,
    /// Payer
    pub payer: solana_program::pubkey::Pubkey,
    /// SPL token program
    pub token_program: solana_program::pubkey::Pubkey,
    /// System program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl CreateCurrencyV1 {
    pub fn instruction(
        &self,
        args: CreateCurrencyV1InstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateCurrencyV1InstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.config, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.currency, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.mint, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(self.authority, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.payer, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateCurrencyV1InstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::OPTIMISTIC_ORACLE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct CreateCurrencyV1InstructionData {
    discriminator: u8,
}

impl CreateCurrencyV1InstructionData {
    pub fn new() -> Self {
        Self { discriminator: 4 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateCurrencyV1InstructionArgs {
    pub reward_range: Bounds,
    pub bond_range: Bounds,
}

/// Instruction builder for `CreateCurrencyV1`.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` currency
///   2. `[]` mint
///   3. `[]` authority
///   4. `[]` payer
///   5. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   6. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct CreateCurrencyV1Builder {
    config: Option<solana_program::pubkey::Pubkey>,
    currency: Option<solana_program::pubkey::Pubkey>,
    mint: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    reward_range: Option<Bounds>,
    bond_range: Option<Bounds>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateCurrencyV1Builder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Config
    #[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.config = Some(config);
        self
    }
    /// Currency
    #[inline(always)]
    pub fn currency(&mut self, currency: solana_program::pubkey::Pubkey) -> &mut Self {
        self.currency = Some(currency);
        self
    }
    /// Mint
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }
    /// Oracle authority
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    /// SPL token program
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// System program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn reward_range(&mut self, reward_range: Bounds) -> &mut Self {
        self.reward_range = Some(reward_range);
        self
    }
    #[inline(always)]
    pub fn bond_range(&mut self, bond_range: Bounds) -> &mut Self {
        self.bond_range = Some(bond_range);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = CreateCurrencyV1 {
            config: self.config.expect("config is not set"),
            currency: self.currency.expect("currency is not set"),
            mint: self.mint.expect("mint is not set"),
            authority: self.authority.expect("authority is not set"),
            payer: self.payer.expect("payer is not set"),
            token_program: self
                .token_program
                .unwrap_or(solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = CreateCurrencyV1InstructionArgs {
            reward_range: self.reward_range.clone().expect("reward_range is not set"),
            bond_range: self.bond_range.clone().expect("bond_range is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_currency_v1` CPI accounts.
pub struct CreateCurrencyV1CpiAccounts<'a, 'b> {
    /// Config
    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Currency
    pub currency: &'b solana_program::account_info::AccountInfo<'a>,
    /// Mint
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Oracle authority
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL token program
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_currency_v1` CPI instruction.
pub struct CreateCurrencyV1Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Config
    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Currency
    pub currency: &'b solana_program::account_info::AccountInfo<'a>,
    /// Mint
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Oracle authority
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL token program
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateCurrencyV1InstructionArgs,
}

impl<'a, 'b> CreateCurrencyV1Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateCurrencyV1CpiAccounts<'a, 'b>,
        args: CreateCurrencyV1InstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            config: accounts.config,
            currency: accounts.currency,
            mint: accounts.mint,
            authority: accounts.authority,
            payer: accounts.payer,
            token_program: accounts.token_program,
            system_program: accounts.system_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.config.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.currency.key, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.mint.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            false,
        ));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.payer.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CreateCurrencyV1InstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::OPTIMISTIC_ORACLE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.currency.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `CreateCurrencyV1` via CPI.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[writable]` currency
///   2. `[]` mint
///   3. `[]` authority
///   4. `[]` payer
///   5. `[]` token_program
///   6. `[]` system_program
#[derive(Clone, Debug)]
pub struct CreateCurrencyV1CpiBuilder<'a, 'b> {
    instruction: Box<CreateCurrencyV1CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateCurrencyV1CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateCurrencyV1CpiBuilderInstruction {
            __program: program,
            config: None,
            currency: None,
            mint: None,
            authority: None,
            payer: None,
            token_program: None,
            system_program: None,
            reward_range: None,
            bond_range: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Config
    #[inline(always)]
    pub fn config(
        &mut self,
        config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.config = Some(config);
        self
    }
    /// Currency
    #[inline(always)]
    pub fn currency(
        &mut self,
        currency: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.currency = Some(currency);
        self
    }
    /// Mint
    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }
    /// Oracle authority
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// SPL token program
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    /// System program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn reward_range(&mut self, reward_range: Bounds) -> &mut Self {
        self.instruction.reward_range = Some(reward_range);
        self
    }
    #[inline(always)]
    pub fn bond_range(&mut self, bond_range: Bounds) -> &mut Self {
        self.instruction.bond_range = Some(bond_range);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> &mut Self {
        self.instruction.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = CreateCurrencyV1InstructionArgs {
            reward_range: self.instruction.reward_range.clone().expect("reward_range is not set"),
            bond_range: self.instruction.bond_range.clone().expect("bond_range is not set"),
        };
        let instruction = CreateCurrencyV1Cpi {
            __program: self.instruction.__program,

            config: self.instruction.config.expect("config is not set"),

            currency: self.instruction.currency.expect("currency is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            token_program: self.instruction.token_program.expect("token_program is not set"),

            system_program: self.instruction.system_program.expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct CreateCurrencyV1CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    currency: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_range: Option<Bounds>,
    bond_range: Option<Bounds>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}
