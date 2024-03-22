//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::CreateRequestArgs;
use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct CreateRequest {
    /// Program oracle account
    pub oracle: solana_program::pubkey::Pubkey,
    /// Request
    pub request: solana_program::pubkey::Pubkey,
    /// Reward mint
    pub reward_mint: solana_program::pubkey::Pubkey,
    /// Reward source token account
    pub reward_source: solana_program::pubkey::Pubkey,
    /// Reward escrow token account
    pub reward_escrow: solana_program::pubkey::Pubkey,
    /// Creator
    pub creator: solana_program::pubkey::Pubkey,
    /// Payer
    pub payer: solana_program::pubkey::Pubkey,
    /// SPL token program
    pub token_program: solana_program::pubkey::Pubkey,
    /// System program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl CreateRequest {
    pub fn instruction(
        &self,
        args: CreateRequestInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateRequestInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(self.oracle, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.request, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(self.reward_mint, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.reward_source, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.reward_escrow, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.creator, true));
        accounts.push(solana_program::instruction::AccountMeta::new(self.payer, true));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateRequestInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction { program_id: crate::ORACLE_ID, accounts, data }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct CreateRequestInstructionData {
    discriminator: u8,
}

impl CreateRequestInstructionData {
    fn new() -> Self {
        Self { discriminator: 1 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateRequestInstructionArgs {
    pub create_request_args: CreateRequestArgs,
}

/// Instruction builder for `CreateRequest`.
///
/// ### Accounts:
///
///   0. `[writable]` oracle
///   1. `[writable]` request
///   2. `[]` reward_mint
///   3. `[writable]` reward_source
///   4. `[writable]` reward_escrow
///   5. `[signer]` creator
///   6. `[writable, signer]` payer
///   7. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   8. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Default)]
pub struct CreateRequestBuilder {
    oracle: Option<solana_program::pubkey::Pubkey>,
    request: Option<solana_program::pubkey::Pubkey>,
    reward_mint: Option<solana_program::pubkey::Pubkey>,
    reward_source: Option<solana_program::pubkey::Pubkey>,
    reward_escrow: Option<solana_program::pubkey::Pubkey>,
    creator: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    create_request_args: Option<CreateRequestArgs>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Program oracle account
    #[inline(always)]
    pub fn oracle(&mut self, oracle: solana_program::pubkey::Pubkey) -> &mut Self {
        self.oracle = Some(oracle);
        self
    }
    /// Request
    #[inline(always)]
    pub fn request(&mut self, request: solana_program::pubkey::Pubkey) -> &mut Self {
        self.request = Some(request);
        self
    }
    /// Reward mint
    #[inline(always)]
    pub fn reward_mint(&mut self, reward_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_mint = Some(reward_mint);
        self
    }
    /// Reward source token account
    #[inline(always)]
    pub fn reward_source(&mut self, reward_source: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_source = Some(reward_source);
        self
    }
    /// Reward escrow token account
    #[inline(always)]
    pub fn reward_escrow(&mut self, reward_escrow: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_escrow = Some(reward_escrow);
        self
    }
    /// Creator
    #[inline(always)]
    pub fn creator(&mut self, creator: solana_program::pubkey::Pubkey) -> &mut Self {
        self.creator = Some(creator);
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
    pub fn create_request_args(&mut self, create_request_args: CreateRequestArgs) -> &mut Self {
        self.create_request_args = Some(create_request_args);
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
        let accounts = CreateRequest {
            oracle: self.oracle.expect("oracle is not set"),
            request: self.request.expect("request is not set"),
            reward_mint: self.reward_mint.expect("reward_mint is not set"),
            reward_source: self.reward_source.expect("reward_source is not set"),
            reward_escrow: self.reward_escrow.expect("reward_escrow is not set"),
            creator: self.creator.expect("creator is not set"),
            payer: self.payer.expect("payer is not set"),
            token_program: self
                .token_program
                .unwrap_or(solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = CreateRequestInstructionArgs {
            create_request_args: self
                .create_request_args
                .clone()
                .expect("create_request_args is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_request` CPI accounts.
pub struct CreateRequestCpiAccounts<'a, 'b> {
    /// Program oracle account
    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,
    /// Request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Reward mint
    pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Reward source token account
    pub reward_source: &'b solana_program::account_info::AccountInfo<'a>,
    /// Reward escrow token account
    pub reward_escrow: &'b solana_program::account_info::AccountInfo<'a>,
    /// Creator
    pub creator: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL token program
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create_request` CPI instruction.
pub struct CreateRequestCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Program oracle account
    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,
    /// Request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Reward mint
    pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Reward source token account
    pub reward_source: &'b solana_program::account_info::AccountInfo<'a>,
    /// Reward escrow token account
    pub reward_escrow: &'b solana_program::account_info::AccountInfo<'a>,
    /// Creator
    pub creator: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL token program
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateRequestInstructionArgs,
}

impl<'a, 'b> CreateRequestCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateRequestCpiAccounts<'a, 'b>,
        args: CreateRequestInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            oracle: accounts.oracle,
            request: accounts.request,
            reward_mint: accounts.reward_mint,
            reward_source: accounts.reward_source,
            reward_escrow: accounts.reward_escrow,
            creator: accounts.creator,
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
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(*self.oracle.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.request.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_mint.key,
            false,
        ));
        accounts
            .push(solana_program::instruction::AccountMeta::new(*self.reward_source.key, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new(*self.reward_escrow.key, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.creator.key, true));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.payer.key, true));
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
        let mut data = CreateRequestInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ORACLE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.oracle.clone());
        account_infos.push(self.request.clone());
        account_infos.push(self.reward_mint.clone());
        account_infos.push(self.reward_source.clone());
        account_infos.push(self.reward_escrow.clone());
        account_infos.push(self.creator.clone());
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

/// Instruction builder for `CreateRequest` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` oracle
///   1. `[writable]` request
///   2. `[]` reward_mint
///   3. `[writable]` reward_source
///   4. `[writable]` reward_escrow
///   5. `[signer]` creator
///   6. `[writable, signer]` payer
///   7. `[]` token_program
///   8. `[]` system_program
pub struct CreateRequestCpiBuilder<'a, 'b> {
    instruction: Box<CreateRequestCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateRequestCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateRequestCpiBuilderInstruction {
            __program: program,
            oracle: None,
            request: None,
            reward_mint: None,
            reward_source: None,
            reward_escrow: None,
            creator: None,
            payer: None,
            token_program: None,
            system_program: None,
            create_request_args: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Program oracle account
    #[inline(always)]
    pub fn oracle(
        &mut self,
        oracle: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.oracle = Some(oracle);
        self
    }
    /// Request
    #[inline(always)]
    pub fn request(
        &mut self,
        request: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.request = Some(request);
        self
    }
    /// Reward mint
    #[inline(always)]
    pub fn reward_mint(
        &mut self,
        reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_mint = Some(reward_mint);
        self
    }
    /// Reward source token account
    #[inline(always)]
    pub fn reward_source(
        &mut self,
        reward_source: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_source = Some(reward_source);
        self
    }
    /// Reward escrow token account
    #[inline(always)]
    pub fn reward_escrow(
        &mut self,
        reward_escrow: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_escrow = Some(reward_escrow);
        self
    }
    /// Creator
    #[inline(always)]
    pub fn creator(
        &mut self,
        creator: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.creator = Some(creator);
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
    pub fn create_request_args(&mut self, create_request_args: CreateRequestArgs) -> &mut Self {
        self.instruction.create_request_args = Some(create_request_args);
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
        let args = CreateRequestInstructionArgs {
            create_request_args: self
                .instruction
                .create_request_args
                .clone()
                .expect("create_request_args is not set"),
        };
        let instruction = CreateRequestCpi {
            __program: self.instruction.__program,

            oracle: self.instruction.oracle.expect("oracle is not set"),

            request: self.instruction.request.expect("request is not set"),

            reward_mint: self.instruction.reward_mint.expect("reward_mint is not set"),

            reward_source: self.instruction.reward_source.expect("reward_source is not set"),

            reward_escrow: self.instruction.reward_escrow.expect("reward_escrow is not set"),

            creator: self.instruction.creator.expect("creator is not set"),

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

struct CreateRequestCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    oracle: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    request: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_source: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_escrow: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    creator: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    create_request_args: Option<CreateRequestArgs>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}
