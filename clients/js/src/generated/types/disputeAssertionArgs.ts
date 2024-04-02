/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import type {
  GetDataEnumKind,
  GetDataEnumKindContent,
  Serializer,
} from "@metaplex-foundation/umi/serializers";

import { dataEnum, struct, u64 } from "@metaplex-foundation/umi/serializers";

export type DisputeAssertionArgs = { __kind: "V1"; value: bigint };

export type DisputeAssertionArgsArgs = { __kind: "V1"; value: number | bigint };

export function getDisputeAssertionArgsSerializer(): Serializer<
  DisputeAssertionArgsArgs,
  DisputeAssertionArgs
> {
  return dataEnum<DisputeAssertionArgs>(
    [["V1", struct<GetDataEnumKindContent<DisputeAssertionArgs, "V1">>([["value", u64()]])]],
    { description: "DisputeAssertionArgs" },
  ) as Serializer<DisputeAssertionArgsArgs, DisputeAssertionArgs>;
}

// Data Enum Helpers.
export function disputeAssertionArgs(
  kind: "V1",
  data: GetDataEnumKindContent<DisputeAssertionArgsArgs, "V1">,
): GetDataEnumKind<DisputeAssertionArgsArgs, "V1">;
export function disputeAssertionArgs<K extends DisputeAssertionArgsArgs["__kind"]>(
  kind: K,
  data?: any,
): Extract<DisputeAssertionArgsArgs, { __kind: K }> {
  return Array.isArray(data) ? { __kind: kind, fields: data } : { __kind: kind, ...(data ?? {}) };
}
export function isDisputeAssertionArgs<K extends DisputeAssertionArgs["__kind"]>(
  kind: K,
  value: DisputeAssertionArgs,
): value is DisputeAssertionArgs & { __kind: K } {
  return value.__kind === kind;
}
