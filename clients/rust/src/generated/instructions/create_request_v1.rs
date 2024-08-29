//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use crate::generated::types::RequestData;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Accounts.
pub struct CreateRequestV1 {
    /// Oracle
    pub oracle: solana_program::pubkey::Pubkey,
    /// Config
    pub config: solana_program::pubkey::Pubkey,
    /// Request
    pub request: solana_program::pubkey::Pubkey,
    /// Reward currency
    pub reward_currency: solana_program::pubkey::Pubkey,
    /// Bond currency
    pub bond_currency: solana_program::pubkey::Pubkey,
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

impl CreateRequestV1 {
    pub fn instruction(
        &self,
        args: CreateRequestV1InstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateRequestV1InstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(self.oracle, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.config, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.request, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_currency,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.bond_currency,
            false,
        ));
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
        let mut data = CreateRequestV1InstructionData::new().try_to_vec().unwrap();
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
pub struct CreateRequestV1InstructionData {
    discriminator: u8,
}

impl CreateRequestV1InstructionData {
    pub fn new() -> Self {
        Self { discriminator: 6 }
    }
}

impl Default for CreateRequestV1InstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateRequestV1InstructionArgs {
    pub reward: u64,
    pub bond: u64,
    pub timestamp: i64,
    pub arbitrator: Pubkey,
    pub data: RequestData,
}

/// Instruction builder for `CreateRequestV1`.
///
/// ### Accounts:
///
///   0. `[writable]` oracle
///   1. `[]` config
///   2. `[writable]` request
///   3. `[]` reward_currency
///   4. `[]` bond_currency
///   5. `[]` reward_mint
///   6. `[writable]` reward_source
///   7. `[writable]` reward_escrow
///   8. `[signer]` creator
///   9. `[writable, signer]` payer
///   10. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   11. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct CreateRequestV1Builder {
    oracle: Option<solana_program::pubkey::Pubkey>,
    config: Option<solana_program::pubkey::Pubkey>,
    request: Option<solana_program::pubkey::Pubkey>,
    reward_currency: Option<solana_program::pubkey::Pubkey>,
    bond_currency: Option<solana_program::pubkey::Pubkey>,
    reward_mint: Option<solana_program::pubkey::Pubkey>,
    reward_source: Option<solana_program::pubkey::Pubkey>,
    reward_escrow: Option<solana_program::pubkey::Pubkey>,
    creator: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    reward: Option<u64>,
    bond: Option<u64>,
    timestamp: Option<i64>,
    arbitrator: Option<Pubkey>,
    data: Option<RequestData>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateRequestV1Builder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Oracle
    #[inline(always)]
    pub fn oracle(&mut self, oracle: solana_program::pubkey::Pubkey) -> &mut Self {
        self.oracle = Some(oracle);
        self
    }
    /// Config
    #[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.config = Some(config);
        self
    }
    /// Request
    #[inline(always)]
    pub fn request(&mut self, request: solana_program::pubkey::Pubkey) -> &mut Self {
        self.request = Some(request);
        self
    }
    /// Reward currency
    #[inline(always)]
    pub fn reward_currency(
        &mut self,
        reward_currency: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.reward_currency = Some(reward_currency);
        self
    }
    /// Bond currency
    #[inline(always)]
    pub fn bond_currency(&mut self, bond_currency: solana_program::pubkey::Pubkey) -> &mut Self {
        self.bond_currency = Some(bond_currency);
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
    pub fn reward(&mut self, reward: u64) -> &mut Self {
        self.reward = Some(reward);
        self
    }
    #[inline(always)]
    pub fn bond(&mut self, bond: u64) -> &mut Self {
        self.bond = Some(bond);
        self
    }
    #[inline(always)]
    pub fn timestamp(&mut self, timestamp: i64) -> &mut Self {
        self.timestamp = Some(timestamp);
        self
    }
    #[inline(always)]
    pub fn arbitrator(&mut self, arbitrator: Pubkey) -> &mut Self {
        self.arbitrator = Some(arbitrator);
        self
    }
    #[inline(always)]
    pub fn data(&mut self, data: RequestData) -> &mut Self {
        self.data = Some(data);
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
        let accounts = CreateRequestV1 {
            oracle: self.oracle.expect("oracle is not set"),
            config: self.config.expect("config is not set"),
            request: self.request.expect("request is not set"),
            reward_currency: self.reward_currency.expect("reward_currency is not set"),
            bond_currency: self.bond_currency.expect("bond_currency is not set"),
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
        let args = CreateRequestV1InstructionArgs {
            reward: self.reward.clone().expect("reward is not set"),
            bond: self.bond.clone().expect("bond is not set"),
            timestamp: self.timestamp.clone().expect("timestamp is not set"),
            arbitrator: self.arbitrator.clone().expect("arbitrator is not set"),
            data: self.data.clone().expect("data is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create_request_v1` CPI accounts.
pub struct CreateRequestV1CpiAccounts<'a, 'b> {
    /// Oracle
    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,
    /// Config
    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Reward currency
    pub reward_currency: &'b solana_program::account_info::AccountInfo<'a>,
    /// Bond currency
    pub bond_currency: &'b solana_program::account_info::AccountInfo<'a>,
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

/// `create_request_v1` CPI instruction.
pub struct CreateRequestV1Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Oracle
    pub oracle: &'b solana_program::account_info::AccountInfo<'a>,
    /// Config
    pub config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Reward currency
    pub reward_currency: &'b solana_program::account_info::AccountInfo<'a>,
    /// Bond currency
    pub bond_currency: &'b solana_program::account_info::AccountInfo<'a>,
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
    pub __args: CreateRequestV1InstructionArgs,
}

impl<'a, 'b> CreateRequestV1Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateRequestV1CpiAccounts<'a, 'b>,
        args: CreateRequestV1InstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            oracle: accounts.oracle,
            config: accounts.config,
            request: accounts.request,
            reward_currency: accounts.reward_currency,
            bond_currency: accounts.bond_currency,
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
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(*self.oracle.key, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.config.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.request.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_currency.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.bond_currency.key,
            false,
        ));
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
        let mut data = CreateRequestV1InstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::OPTIMISTIC_ORACLE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(12 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.oracle.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.request.clone());
        account_infos.push(self.reward_currency.clone());
        account_infos.push(self.bond_currency.clone());
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

/// Instruction builder for `CreateRequestV1` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` oracle
///   1. `[]` config
///   2. `[writable]` request
///   3. `[]` reward_currency
///   4. `[]` bond_currency
///   5. `[]` reward_mint
///   6. `[writable]` reward_source
///   7. `[writable]` reward_escrow
///   8. `[signer]` creator
///   9. `[writable, signer]` payer
///   10. `[]` token_program
///   11. `[]` system_program
#[derive(Clone, Debug)]
pub struct CreateRequestV1CpiBuilder<'a, 'b> {
    instruction: Box<CreateRequestV1CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateRequestV1CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateRequestV1CpiBuilderInstruction {
            __program: program,
            oracle: None,
            config: None,
            request: None,
            reward_currency: None,
            bond_currency: None,
            reward_mint: None,
            reward_source: None,
            reward_escrow: None,
            creator: None,
            payer: None,
            token_program: None,
            system_program: None,
            reward: None,
            bond: None,
            timestamp: None,
            arbitrator: None,
            data: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Oracle
    #[inline(always)]
    pub fn oracle(
        &mut self,
        oracle: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.oracle = Some(oracle);
        self
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
    /// Request
    #[inline(always)]
    pub fn request(
        &mut self,
        request: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.request = Some(request);
        self
    }
    /// Reward currency
    #[inline(always)]
    pub fn reward_currency(
        &mut self,
        reward_currency: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_currency = Some(reward_currency);
        self
    }
    /// Bond currency
    #[inline(always)]
    pub fn bond_currency(
        &mut self,
        bond_currency: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.bond_currency = Some(bond_currency);
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
    pub fn reward(&mut self, reward: u64) -> &mut Self {
        self.instruction.reward = Some(reward);
        self
    }
    #[inline(always)]
    pub fn bond(&mut self, bond: u64) -> &mut Self {
        self.instruction.bond = Some(bond);
        self
    }
    #[inline(always)]
    pub fn timestamp(&mut self, timestamp: i64) -> &mut Self {
        self.instruction.timestamp = Some(timestamp);
        self
    }
    #[inline(always)]
    pub fn arbitrator(&mut self, arbitrator: Pubkey) -> &mut Self {
        self.instruction.arbitrator = Some(arbitrator);
        self
    }
    #[inline(always)]
    pub fn data(&mut self, data: RequestData) -> &mut Self {
        self.instruction.data = Some(data);
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
        let args = CreateRequestV1InstructionArgs {
            reward: self.instruction.reward.clone().expect("reward is not set"),
            bond: self.instruction.bond.clone().expect("bond is not set"),
            timestamp: self.instruction.timestamp.clone().expect("timestamp is not set"),
            arbitrator: self.instruction.arbitrator.clone().expect("arbitrator is not set"),
            data: self.instruction.data.clone().expect("data is not set"),
        };
        let instruction = CreateRequestV1Cpi {
            __program: self.instruction.__program,

            oracle: self.instruction.oracle.expect("oracle is not set"),

            config: self.instruction.config.expect("config is not set"),

            request: self.instruction.request.expect("request is not set"),

            reward_currency: self.instruction.reward_currency.expect("reward_currency is not set"),

            bond_currency: self.instruction.bond_currency.expect("bond_currency is not set"),

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

#[derive(Clone, Debug)]
struct CreateRequestV1CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    oracle: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    request: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_currency: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bond_currency: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_source: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_escrow: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    creator: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward: Option<u64>,
    bond: Option<u64>,
    timestamp: Option<i64>,
    arbitrator: Option<Pubkey>,
    data: Option<RequestData>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}
