/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import type { AccountTypeArgs } from "../types";
import type {
  Account,
  Context,
  DateTime,
  DateTimeInput,
  Pda,
  PublicKey,
  RpcAccount,
  RpcGetAccountOptions,
  RpcGetAccountsOptions,
} from "@metaplex-foundation/umi";
import type { Serializer } from "@metaplex-foundation/umi/serializers";

import {
  assertAccountExists,
  deserializeAccount,
  gpaBuilder,
  mapDateTimeSerializer,
  publicKey as toPublicKey,
} from "@metaplex-foundation/umi";
import {
  i64,
  map,
  mapSerializer,
  publicKey as publicKeySerializer,
  string,
  struct,
  u64,
} from "@metaplex-foundation/umi/serializers";

import { AccountType, getAccountTypeSerializer } from "../types";

export type VotingV1 = Account<VotingV1AccountData>;

export interface VotingV1AccountData {
  accountType: AccountType;
  request: PublicKey;
  startTimestamp: DateTime;
  endTimestamp: DateTime;
  voteCount: bigint;
  modeValue: bigint;
  votes: Map<bigint, bigint>;
}

export interface VotingV1AccountDataArgs {
  request: PublicKey;
  startTimestamp: DateTimeInput;
  endTimestamp: DateTimeInput;
  voteCount: number | bigint;
  modeValue: number | bigint;
  votes: Map<number | bigint, number | bigint>;
}

export function getVotingV1AccountDataSerializer(): Serializer<
  VotingV1AccountDataArgs,
  VotingV1AccountData
> {
  return mapSerializer<VotingV1AccountDataArgs, any, VotingV1AccountData>(
    struct<VotingV1AccountData>(
      [
        ["accountType", getAccountTypeSerializer()],
        ["request", publicKeySerializer()],
        ["startTimestamp", mapDateTimeSerializer(i64())],
        ["endTimestamp", mapDateTimeSerializer(i64())],
        ["voteCount", u64()],
        ["modeValue", u64()],
        ["votes", map(u64(), u64())],
      ],
      { description: "VotingV1AccountData" },
    ),
    (value) => ({ ...value, accountType: AccountType.VotingV1 }),
  );
}

export function deserializeVotingV1(rawAccount: RpcAccount): VotingV1 {
  return deserializeAccount(rawAccount, getVotingV1AccountDataSerializer());
}

export async function fetchVotingV1(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<VotingV1> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  assertAccountExists(maybeAccount, "VotingV1");
  return deserializeVotingV1(maybeAccount);
}

export async function safeFetchVotingV1(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<VotingV1 | null> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  return maybeAccount.exists ? deserializeVotingV1(maybeAccount) : null;
}

export async function fetchAllVotingV1(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<VotingV1>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, "VotingV1");
    return deserializeVotingV1(maybeAccount);
  });
}

export async function safeFetchAllVotingV1(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<VotingV1>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializeVotingV1(maybeAccount as RpcAccount));
}

export function getVotingV1GpaBuilder(context: Pick<Context, "rpc" | "programs">) {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      accountType: AccountTypeArgs;
      request: PublicKey;
      startTimestamp: DateTimeInput;
      endTimestamp: DateTimeInput;
      voteCount: number | bigint;
      modeValue: number | bigint;
      votes: Map<number | bigint, number | bigint>;
    }>({
      accountType: [0, getAccountTypeSerializer()],
      request: [1, publicKeySerializer()],
      startTimestamp: [33, mapDateTimeSerializer(i64())],
      endTimestamp: [41, mapDateTimeSerializer(i64())],
      voteCount: [49, u64()],
      modeValue: [57, u64()],
      votes: [65, map(u64(), u64())],
    })
    .deserializeUsing<VotingV1>((account) => deserializeVotingV1(account))
    .whereField("accountType", AccountType.VotingV1);
}

export function findVotingV1Pda(
  context: Pick<Context, "eddsa" | "programs">,
  seeds: {
    /** The address of the request. */
    request: PublicKey;
  },
): Pda {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );
  return context.eddsa.findPda(programId, [
    string({ size: "variable" }).serialize("voting"),
    publicKeySerializer().serialize(seeds.request),
  ]);
}

export async function fetchVotingV1FromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findVotingV1Pda>[1],
  options?: RpcGetAccountOptions,
): Promise<VotingV1> {
  return fetchVotingV1(context, findVotingV1Pda(context, seeds), options);
}

export async function safeFetchVotingV1FromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findVotingV1Pda>[1],
  options?: RpcGetAccountOptions,
): Promise<VotingV1 | null> {
  return safeFetchVotingV1(context, findVotingV1Pda(context, seeds), options);
}