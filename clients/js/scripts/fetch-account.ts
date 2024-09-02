import type {
  AssertionV1,
  ConfigV1,
  CurrencyV1,
  OracleV1,
  RequestV1,
  StakeV1,
  VoteV1,
  VotingV1,
} from "../src";
import type { RpcAccount } from "@metaplex-foundation/umi";

import { isPublicKey } from "@metaplex-foundation/umi";
import { bold } from "colorette";

import {
  AccountType,
  deserializeAssertionV1,
  deserializeConfigV1,
  deserializeCurrencyV1,
  deserializeOracleV1,
  deserializeRequestV1,
  deserializeStakeV1,
  deserializeVoteV1,
  deserializeVotingV1,
  getAccountTypeSerializer,
  getOptimisticOracleProgramId,
} from "../src";

import { MAINNET_URL, createUmi, error } from "./_utils";

const args = process.argv.slice(2);

const address = args[0]?.trim();

if (address === undefined) {
  error("Missing account address argument");
} else if (!isPublicKey(address)) {
  error(`'${address}' is not a valid address`);
}

const rpcUrl = process.env.RPC_URL ?? MAINNET_URL;
const umi = createUmi(rpcUrl);

console.log(`${bold("Cluster:")} ${umi.rpc.getCluster()}`);
console.log(`${bold("Endpoint:")} ${umi.rpc.getEndpoint()}`);
console.log();

const account = await umi.rpc.getAccount(address);

if (!account.exists) {
  error(`Account [${address}] does not exist`);
}

const data = deserializeAccount(account);

console.log(bold(AccountType[data.accountType]));
console.dir(data, { colors: true, depth: null });

process.exit(0);

function deserializeAccount(
  account: RpcAccount,
): OracleV1 | ConfigV1 | StakeV1 | RequestV1 | AssertionV1 | CurrencyV1 | VotingV1 | VoteV1 {
  if (account.owner !== getOptimisticOracleProgramId(umi)) {
    error(`Account [${account.publicKey}] is not owned by the optimistic oracle program`);
  }

  let accountType: AccountType;
  try {
    [accountType] = getAccountTypeSerializer().deserialize(account.data);
  } catch {
    const kind = account.data[0];
    if (kind === undefined) {
      accountType = AccountType.Uninitialized;
    } else {
      error(`Unknown account type: 0x${kind.toString(16)}`);
    }
  }

  switch (accountType) {
    case AccountType.OracleV1:
      return deserializeOracleV1(account);
    case AccountType.ConfigV1:
      return deserializeConfigV1(account);
    case AccountType.StakeV1:
      return deserializeStakeV1(account);
    case AccountType.RequestV1:
      return deserializeRequestV1(account);
    case AccountType.AssertionV1:
      return deserializeAssertionV1(account);
    case AccountType.CurrencyV1:
      return deserializeCurrencyV1(account);
    case AccountType.VotingV1:
      return deserializeVotingV1(account);
    case AccountType.VoteV1:
      return deserializeVoteV1(account);

    case AccountType.Uninitialized:
      error("Account is uninitialized");
  }
}
