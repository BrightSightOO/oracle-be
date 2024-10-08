/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import type { ResolvedAccount, ResolvedAccountsWithIndices } from "../shared";
import type { Context, Pda, PublicKey, TransactionBuilder } from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import { transactionBuilder } from "@metaplex-foundation/umi";
import { mapSerializer, struct, u8 } from "@metaplex-foundation/umi/serializers";

import { findVotingV1Pda } from "../accounts";
import { expectPublicKey, getAccountMetasAndSigners } from "../shared";

// Accounts.
export type CloseVotingV1InstructionAccounts = {
  /** Config */
  config: PublicKey | Pda;
  /** Request */
  request: PublicKey | Pda;
  /** Voting */
  voting?: PublicKey | Pda;
};

// Data.
export type CloseVotingV1InstructionData = { discriminator: number };

export type CloseVotingV1InstructionDataArgs = {};

export function getCloseVotingV1InstructionDataSerializer(): Serializer<
  CloseVotingV1InstructionDataArgs,
  CloseVotingV1InstructionData
> {
  return mapSerializer<CloseVotingV1InstructionDataArgs, any, CloseVotingV1InstructionData>(
    struct<CloseVotingV1InstructionData>([["discriminator", u8()]], {
      description: "CloseVotingV1InstructionData",
    }),
    (value) => ({ ...value, discriminator: 11 }),
  );
}

// Instruction.
export function closeVotingV1(
  context: Pick<Context, "eddsa" | "programs">,
  input: CloseVotingV1InstructionAccounts,
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
      isWritable: false as boolean,
      value: input.config ?? null,
    },
    request: {
      index: 1,
      isWritable: true as boolean,
      value: input.request ?? null,
    },
    voting: {
      index: 2,
      isWritable: true as boolean,
      value: input.voting ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Default values.
  if (!resolvedAccounts.voting.value) {
    resolvedAccounts.voting.value = findVotingV1Pda(context, {
      request: expectPublicKey(resolvedAccounts.request.value),
    });
  }

  // Accounts in order.
  const orderedAccounts: Array<ResolvedAccount> = Object.values(resolvedAccounts).sort(
    (a, b) => a.index - b.index,
  );

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(orderedAccounts, "programId", programId);

  // Data.
  const data = getCloseVotingV1InstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
