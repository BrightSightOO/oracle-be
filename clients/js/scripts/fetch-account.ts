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

import { createUmi, installErrorHandler, logger, parseCliArgs, readCliConfig } from "./utils";

installErrorHandler();

const argv = parseCliArgs(
  {
    config: {
      type: "string",
      desc: "Configuration file to use",
    },
    url: {
      type: "string",
      alias: "u",
      desc: "URL for Solana's JSON RPC or moniker",
      valueName: "URL",
    },
    ws: {
      type: "string",
      desc: "WebSocket URL for the solana cluster",
      valueName: "URL",
    },
    verbose: {
      type: "boolean",
      alias: "v",
      default: false,
      desc: "Increase verbosity of output",
    },
  },
  [{ name: "address", required: true }],
);

let { url: rpcUrl } = argv;

const [address] = argv._;

if (!isPublicKey(address)) {
  logger.bail(`Address [${address}] is not a valid public key`);
}

if (rpcUrl === undefined) {
  const config = await readCliConfig(argv.config);

  rpcUrl ??= config.rpcUrl;
}

const umi = createUmi(rpcUrl, argv.ws);

//////////////////////////////////////////////////

logger.entry("Cluster", umi.rpc.getCluster());
logger.entry("Endpoint", umi.rpc.getEndpoint());
logger.newline();

//////////////////////////////////////////////////

const account = await umi.rpc.getAccount(address);

if (!account.exists) {
  logger.bail(`Account [${address}] does not exist`);
}

const { accountType, publicKey, header, ...data } = deserializeAccount(account);

if (argv.verbose) {
  logger.entry("Address", publicKey);
  logger.group("Header", (group) => {
    for (const [key, value] of Object.entries(header)) {
      group.entry(key, value);
    }
  });
  logger.newline();
}

logger.group(AccountType[accountType], (group) => {
  for (const [key, value] of Object.entries(data)) {
    group.entry(key, value);
  }
});

process.exit(0);

function deserializeAccount(
  account: RpcAccount,
): OracleV1 | ConfigV1 | StakeV1 | RequestV1 | AssertionV1 | CurrencyV1 | VotingV1 | VoteV1 {
  if (account.owner !== getOptimisticOracleProgramId(umi)) {
    logger.bail(`Account [${account.publicKey}] is not owned by the optimistic oracle program`);
  }

  let accountType: AccountType;
  try {
    [accountType] = getAccountTypeSerializer().deserialize(account.data);
  } catch {
    const kind = account.data[0];
    if (kind === undefined) {
      accountType = AccountType.Uninitialized;
    } else {
      logger.bail(`Account type [0x${kind.toString(16)}] is not recognized`);
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
      logger.bail("Account is uninitialized");
  }
}
