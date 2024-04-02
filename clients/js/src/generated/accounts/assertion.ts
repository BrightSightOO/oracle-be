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

export type Assertion = Account<AssertionAccountData>;

export type AssertionAccountData = {
  accountType: AccountType;
  request: PublicKey;
  assertionTimestamp: bigint;
  expirationTimestamp: bigint;
  asserter: PublicKey;
  disputer: PublicKey;
  assertedValue: bigint;
  disputedValue: bigint;
};

export type AssertionAccountDataArgs = {
  request: PublicKey;
  assertionTimestamp: number | bigint;
  expirationTimestamp: number | bigint;
  asserter: PublicKey;
  disputer: PublicKey;
  assertedValue: number | bigint;
  disputedValue: number | bigint;
};

export function getAssertionAccountDataSerializer(): Serializer<
  AssertionAccountDataArgs,
  AssertionAccountData
> {
  return mapSerializer<AssertionAccountDataArgs, any, AssertionAccountData>(
    struct<AssertionAccountData>(
      [
        ["accountType", getAccountTypeSerializer()],
        ["request", publicKeySerializer()],
        ["assertionTimestamp", i64()],
        ["expirationTimestamp", i64()],
        ["asserter", publicKeySerializer()],
        ["disputer", publicKeySerializer()],
        ["assertedValue", u64()],
        ["disputedValue", u64()],
      ],
      { description: "AssertionAccountData" },
    ),
    (value) => ({ ...value, accountType: AccountType.Assertion }),
  );
}

export function deserializeAssertion(rawAccount: RpcAccount): Assertion {
  return deserializeAccount(rawAccount, getAssertionAccountDataSerializer());
}

export async function fetchAssertion(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<Assertion> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  assertAccountExists(maybeAccount, "Assertion");
  return deserializeAssertion(maybeAccount);
}

export async function safeFetchAssertion(
  context: Pick<Context, "rpc">,
  publicKey: PublicKey | Pda,
  options?: RpcGetAccountOptions,
): Promise<Assertion | null> {
  const maybeAccount = await context.rpc.getAccount(toPublicKey(publicKey, false), options);
  return maybeAccount.exists ? deserializeAssertion(maybeAccount) : null;
}

export async function fetchAllAssertion(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<Assertion>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts.map((maybeAccount) => {
    assertAccountExists(maybeAccount, "Assertion");
    return deserializeAssertion(maybeAccount);
  });
}

export async function safeFetchAllAssertion(
  context: Pick<Context, "rpc">,
  publicKeys: Array<PublicKey | Pda>,
  options?: RpcGetAccountsOptions,
): Promise<Array<Assertion>> {
  const maybeAccounts = await context.rpc.getAccounts(
    publicKeys.map((key) => toPublicKey(key, false)),
    options,
  );
  return maybeAccounts
    .filter((maybeAccount) => maybeAccount.exists)
    .map((maybeAccount) => deserializeAssertion(maybeAccount as RpcAccount));
}

export function getAssertionGpaBuilder(context: Pick<Context, "rpc" | "programs">) {
  const programId = context.programs.getPublicKey(
    "optimisticOracle",
    "DVMysqEbKDZdaJ1AVcmAqyVfvvZAMFwUkEQsNMQTvMZg",
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      accountType: AccountTypeArgs;
      request: PublicKey;
      assertionTimestamp: number | bigint;
      expirationTimestamp: number | bigint;
      asserter: PublicKey;
      disputer: PublicKey;
      assertedValue: number | bigint;
      disputedValue: number | bigint;
    }>({
      accountType: [0, getAccountTypeSerializer()],
      request: [1, publicKeySerializer()],
      assertionTimestamp: [33, i64()],
      expirationTimestamp: [41, i64()],
      asserter: [49, publicKeySerializer()],
      disputer: [81, publicKeySerializer()],
      assertedValue: [113, u64()],
      disputedValue: [121, u64()],
    })
    .deserializeUsing<Assertion>((account) => deserializeAssertion(account))
    .whereField("accountType", AccountType.Assertion);
}

export function getAssertionSize(): number {
  return 129;
}

export function findAssertionPda(
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

export async function fetchAssertionFromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findAssertionPda>[1],
  options?: RpcGetAccountOptions,
): Promise<Assertion> {
  return fetchAssertion(context, findAssertionPda(context, seeds), options);
}

export async function safeFetchAssertionFromSeeds(
  context: Pick<Context, "eddsa" | "programs" | "rpc">,
  seeds: Parameters<typeof findAssertionPda>[1],
  options?: RpcGetAccountOptions,
): Promise<Assertion | null> {
  return safeFetchAssertion(context, findAssertionPda(context, seeds), options);
}
