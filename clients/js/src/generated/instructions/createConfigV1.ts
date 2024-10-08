/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import type { ResolvedAccount, ResolvedAccountsWithIndices } from "../shared";
import type { Context, Pda, PublicKey, Signer, TransactionBuilder } from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { transactionBuilder } from "@metaplex-foundation/umi";
import {
  mapSerializer,
  publicKey as publicKeySerializer,
  struct,
  u16,
  u32,
  u8,
} from "@metaplex-foundation/umi/serializers";

import { getAccountMetasAndSigners } from "../shared";

// Accounts.
export type CreateConfigV1InstructionAccounts = {
  /** Config */
  config: Signer;
  /** Payer */
  payer?: Signer;
  /** System program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type CreateConfigV1InstructionData = {
  discriminator: number;
  authority: PublicKey;
  bondFeeBps: number;
  disputeWindow: number;
  votingWindow: number;
  arbitrationWindow: number;
};

export type CreateConfigV1InstructionDataArgs = {
  authority: PublicKey;
  bondFeeBps: number;
  disputeWindow: number;
  votingWindow: number;
  arbitrationWindow: number;
};

export function getCreateConfigV1InstructionDataSerializer(): Serializer<
  CreateConfigV1InstructionDataArgs,
  CreateConfigV1InstructionData
> {
  return mapSerializer<CreateConfigV1InstructionDataArgs, any, CreateConfigV1InstructionData>(
    struct<CreateConfigV1InstructionData>(
      [
        ["discriminator", u8()],
        ["authority", publicKeySerializer()],
        ["bondFeeBps", u16()],
        ["disputeWindow", u32()],
        ["votingWindow", u32()],
        ["arbitrationWindow", u32()],
      ],
      { description: "CreateConfigV1InstructionData" },
    ),
    (value) => ({ ...value, discriminator: 2 }),
  );
}

// Args.
export type CreateConfigV1InstructionArgs = CreateConfigV1InstructionDataArgs;

// Instruction.
export function createConfigV1(
  context: Pick<Context, "payer" | "programs">,
  input: CreateConfigV1InstructionAccounts & CreateConfigV1InstructionArgs,
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVM2j1a1AJ9hZuEXyMxA5vusKgMR2FcKJyCf3QE5R2ge",
  );

  // Accounts.
  const resolvedAccounts = {
    config: {
      index: 0,
      isWritable: true as boolean,
      value: input.config ?? null,
    },
    payer: {
      index: 1,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    systemProgram: {
      index: 2,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: CreateConfigV1InstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer;
  }
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      "splSystem",
      "11111111111111111111111111111111",
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: Array<ResolvedAccount> = Object.values(resolvedAccounts).sort(
    (a, b) => a.index - b.index,
  );

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(orderedAccounts, "programId", programId);

  // Data.
  const data = getCreateConfigV1InstructionDataSerializer().serialize(resolvedArgs);

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
