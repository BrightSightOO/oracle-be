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
import { mapSerializer, struct, u8 } from "@metaplex-foundation/umi/serializers";

import { findAssertionV1Pda } from "../accounts";
import { expectPublicKey, getAccountMetasAndSigners } from "../shared";

// Accounts.
export type ClaimAssertionV1InstructionAccounts = {
  /** Request */
  request: PublicKey | Pda;
  /** Assertion */
  assertion?: PublicKey | Pda;
  /** Bond mint */
  bondMint: PublicKey | Pda;
  /** Reclaimed bond destination token account */
  bondDestination: PublicKey | Pda;
  /** Asserter bond escrow token account */
  bondEscrow: PublicKey | Pda;
  /** Reward mint */
  rewardMint: PublicKey | Pda;
  /** Reward destination token account */
  rewardDestination: PublicKey | Pda;
  /** Reward escrow token account */
  rewardEscrow: PublicKey | Pda;
  /** Asserter */
  asserter: Signer;
  /** SPL token program */
  tokenProgram?: PublicKey | Pda;
  /** System program */
  systemProgram?: PublicKey | Pda;
};

// Data.
export type ClaimAssertionV1InstructionData = { discriminator: number };

export type ClaimAssertionV1InstructionDataArgs = {};

export function getClaimAssertionV1InstructionDataSerializer(): Serializer<
  ClaimAssertionV1InstructionDataArgs,
  ClaimAssertionV1InstructionData
> {
  return mapSerializer<ClaimAssertionV1InstructionDataArgs, any, ClaimAssertionV1InstructionData>(
    struct<ClaimAssertionV1InstructionData>([["discriminator", u8()]], {
      description: "ClaimAssertionV1InstructionData",
    }),
    (value) => ({ ...value, discriminator: 13 }),
  );
}

// Instruction.
export function claimAssertionV1(
  context: Pick<Context, "eddsa" | "programs">,
  input: ClaimAssertionV1InstructionAccounts,
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );

  // Accounts.
  const resolvedAccounts = {
    request: {
      index: 0,
      isWritable: false as boolean,
      value: input.request ?? null,
    },
    assertion: {
      index: 1,
      isWritable: true as boolean,
      value: input.assertion ?? null,
    },
    bondMint: {
      index: 2,
      isWritable: false as boolean,
      value: input.bondMint ?? null,
    },
    bondDestination: {
      index: 3,
      isWritable: true as boolean,
      value: input.bondDestination ?? null,
    },
    bondEscrow: {
      index: 4,
      isWritable: true as boolean,
      value: input.bondEscrow ?? null,
    },
    rewardMint: {
      index: 5,
      isWritable: false as boolean,
      value: input.rewardMint ?? null,
    },
    rewardDestination: {
      index: 6,
      isWritable: true as boolean,
      value: input.rewardDestination ?? null,
    },
    rewardEscrow: {
      index: 7,
      isWritable: true as boolean,
      value: input.rewardEscrow ?? null,
    },
    asserter: {
      index: 8,
      isWritable: true as boolean,
      value: input.asserter ?? null,
    },
    tokenProgram: {
      index: 9,
      isWritable: false as boolean,
      value: input.tokenProgram ?? null,
    },
    systemProgram: {
      index: 10,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Default values.
  if (!resolvedAccounts.assertion.value) {
    resolvedAccounts.assertion.value = findAssertionV1Pda(context, {
      request: expectPublicKey(resolvedAccounts.request.value),
    });
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
  const data = getClaimAssertionV1InstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
