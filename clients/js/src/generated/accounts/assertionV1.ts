/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
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
  mapSerializer,
  publicKey as publicKeySerializer,
  string,
  struct,
  u64,
} from "@metaplex-foundation/umi/serializers";

import { AccountType, getAccountTypeSerializer } from "../types";

export type AssertionV1 = Account<AssertionV1AccountData>;

export type AssertionV1AccountData = {
  accountType: AccountType;
  request: PublicKey;
  assertionTimestamp: DateTime;
  expirationTimestamp: DateTime;
  asserter: PublicKey;
  disputer: PublicKey;
  assertedValue: bigint;
};

export type AssertionV1AccountDataArgs = {
  request: PublicKey;
  assertionTimestamp: DateTimeInput;
  expirationTimestamp: DateTimeInput;
  asserter: PublicKey;
  disputer: PublicKey;
  assertedValue: number | bigint;
};

export function getAssertionV1AccountDataSerializer(): Serializer<
  AssertionV1AccountDataArgs,
  AssertionV1AccountData
> {
  return mapSerializer<AssertionV1AccountDataArgs, any, AssertionV1AccountData>(
    struct<AssertionV1AccountData>(
      [
        ["accountType", getAccountTypeSerializer()],
        ["request", publicKeySerializer()],
        ["assertionTimestamp", mapDateTimeSerializer(i64())],
        ["expirationTimestamp", mapDateTimeSerializer(i64())],
        ["asserter", publicKeySerializer()],
        ["disputer", publicKeySerializer()],
        ["assertedValue", u64()],
      ],
      { description: "AssertionV1AccountData" },
    ),
    (value) => ({ ...value, accountType: AccountType.AssertionV1 }),
  );
}

export function deserializeAssertionV1(rawAccount: RpcAccount): AssertionV1 {
  return deserializeAccount(rawAccount, getAssertionV1AccountDataSerializer());
}

export async function fetchAssertionV1(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<AssertionV1> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  assertAccountExists(maybeAccount, "AssertionV1");
  return deserializeAssertionV1(maybeAccount);
}

export async function safeFetchAssertionV1(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<AssertionV1 | null> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  return maybeAccount.exists ? deserializeAssertionV1(maybeAccount) : null;
}

export async function fetchAllAssertionV1(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<AssertionV1>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, "AssertionV1");
    return deserializeAssertionV1(maybeAccount);
  });
}

export async function safeFetchAllAssertionV1(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<AssertionV1>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializeAssertionV1(maybeAccount as RpcAccount));
}

export function getAssertionV1GpaBuilder(context: Pick<Context, "rpc" | "programs">) {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      accountType: AccountTypeArgs;
      request: PublicKey;
      assertionTimestamp: DateTimeInput;
      expirationTimestamp: DateTimeInput;
      asserter: PublicKey;
      disputer: PublicKey;
      assertedValue: number | bigint;
    }>({
      accountType: [0, getAccountTypeSerializer()],
      request: [1, publicKeySerializer()],
      assertionTimestamp: [33, mapDateTimeSerializer(i64())],
      expirationTimestamp: [41, mapDateTimeSerializer(i64())],
      asserter: [49, publicKeySerializer()],
      disputer: [81, publicKeySerializer()],
      assertedValue: [113, u64()],
    })
    .deserializeUsing<AssertionV1>((account) => deserializeAssertionV1(account))
    .whereField("accountType", AccountType.AssertionV1);
}

export function getAssertionV1Size(): number {
  return 121;
}

export function findAssertionV1Pda(
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
    string({ size: "variable" }).serialize("assertion"),
    publicKeySerializer().serialize(seeds.request),
  ]);
}

export async function fetchAssertionV1FromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findAssertionV1Pda>[1],
  options?: RpcGetAccountOptions,
): Promise<AssertionV1> {
  return fetchAssertionV1(context, findAssertionV1Pda(context, seeds), options);
}

export async function safeFetchAssertionV1FromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findAssertionV1Pda>[1],
  options?: RpcGetAccountOptions,
): Promise<AssertionV1 | null> {
  return safeFetchAssertionV1(context, findAssertionV1Pda(context, seeds), options);
}
