/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import type { ResolvedAccount, ResolvedAccountsWithIndices } from "../shared";
import type { Bounds, BoundsArgs } from "../types";
import type { Context, Pda, PublicKey, TransactionBuilder } from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { transactionBuilder } from "@metaplex-foundation/umi";
import { mapSerializer, struct, u8 } from "@metaplex-foundation/umi/serializers";

import { getAccountMetasAndSigners } from "../shared";
import { getBoundsSerializer } from "../types";

// Accounts.
export interface CreateCurrencyV1InstructionAccounts {
  /** Config */
  config: PublicKey | Pda;
  /** Currency */
  currency: PublicKey | Pda;
  /** Mint */
  mint: PublicKey | Pda;
  /** Oracle authority */
  authority?: PublicKey | Pda;
  /** Payer */
  payer?: PublicKey | Pda;
  /** SPL token program */
  tokenProgram?: PublicKey | Pda;
  /** System program */
  systemProgram?: PublicKey | Pda;
}

// Data.
export interface CreateCurrencyV1InstructionData {
  discriminator: number;
  rewardRange: Bounds;
  bondRange: Bounds;
}

export interface CreateCurrencyV1InstructionDataArgs {
  rewardRange: BoundsArgs;
  bondRange: BoundsArgs;
}

export function getCreateCurrencyV1InstructionDataSerializer(): Serializer<
  CreateCurrencyV1InstructionDataArgs,
  CreateCurrencyV1InstructionData
> {
  return mapSerializer<CreateCurrencyV1InstructionDataArgs, any, CreateCurrencyV1InstructionData>(
    struct<CreateCurrencyV1InstructionData>(
      [
        ["discriminator", u8()],
        ["rewardRange", getBoundsSerializer()],
        ["bondRange", getBoundsSerializer()],
      ],
      { description: "CreateCurrencyV1InstructionData" },
    ),
    (value) => ({ ...value, discriminator: 4 }),
  );
}

// Args.
export type CreateCurrencyV1InstructionArgs = CreateCurrencyV1InstructionDataArgs;

// Instruction.
export function createCurrencyV1(
  context: Pick<Context, "identity" | "payer" | "programs">,
  input: CreateCurrencyV1InstructionAccounts & CreateCurrencyV1InstructionArgs,
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );

  // Accounts.
  const resolvedAccounts = {
    config: {
      index: 0,
      isWritable: false as boolean,
      value: input.config ?? null,
    },
    currency: {
      index: 1,
      isWritable: true as boolean,
      value: input.currency ?? null,
    },
    mint: { index: 2, isWritable: false as boolean, value: input.mint ?? null },
    authority: {
      index: 3,
      isWritable: false as boolean,
      value: input.authority ?? null,
    },
    payer: {
      index: 4,
      isWritable: false as boolean,
      value: input.payer ?? null,
    },
    tokenProgram: {
      index: 5,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    systemProgram: {
      index: 6,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: CreateCurrencyV1InstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.authority.value) {
    resolvedAccounts.authority.value = context.identity.publicKey;
  }
  if (!resolvedAccounts.payer.value) {
    resolvedAccounts.payer.value = context.payer.publicKey;
  }
  if (!resolvedAccounts.tokenProgram.value) {
    resolvedAccounts.tokenProgram.value = context.programs.getPublicKey(
      "splToken",
      "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    );
    resolvedAccounts.tokenProgram.isWritable = false;
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
  const data = getCreateCurrencyV1InstructionDataSerializer().serialize(resolvedArgs);

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
