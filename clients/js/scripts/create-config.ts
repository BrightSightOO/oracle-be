import type { Amount, PublicKey } from "@metaplex-foundation/umi";

import {
  amountToString,
  createAmount,
  displayAmount,
  generateSigner,
  isZeroAmount,
  keypairIdentity,
} from "@metaplex-foundation/umi";
import { base58 } from "@metaplex-foundation/umi/serializers";

import { createConfigV1 } from "../src";

import {
  cancel,
  createUmi,
  formatDuration,
  installErrorHandler,
  logger,
  parseCliArgs,
  readCliConfig,
  readKeypair,
  spinner,
} from "./utils";
import * as prompt from "./utils/prompt";

installErrorHandler();

const argv = parseCliArgs({
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
  keypair: {
    type: "string",
    alias: "k",
    desc: "Filepath to a keypair",
  },
  ws: {
    type: "string",
    desc: "WebSocket URL for the solana cluster",
    valueName: "URL",
  },
});

let { url: rpcUrl, keypair: keypairPath } = argv;

if (rpcUrl === undefined || keypairPath === undefined) {
  const config = await readCliConfig(argv.config);

  rpcUrl ??= config.rpcUrl;
  keypairPath ??= config.keypairPath;
}

const keypair = await readKeypair(keypairPath);
const umi = createUmi(rpcUrl, argv.ws).use(keypairIdentity(keypair));

//////////////////////////////////////////////////

logger.entry("Cluster", umi.rpc.getCluster());
logger.entry("Endpoint", umi.rpc.getEndpoint());
logger.newline();

{
  const wallet = umi.identity.publicKey;
  const balance = await umi.rpc.getBalance(wallet);

  logger.group("Wallet", (group) => {
    group.entry("Address", wallet);
    group.entry("Balance", amountToString(balance));
  });
  logger.newline();

  if (isZeroAmount(balance)) {
    logger.bail("Wallet balance is empty, are you using the correct wallet?");
  }
}

//////////////////////////////////////////////////

type ConfigArgs = {
  authority: PublicKey;
  bondFeeBps: Amount<"%", 2>;
  disputeWindow: number;
  votingWindow: number;
  arbitrationWindow: number;
};

const args: ConfigArgs = {
  authority: await prompt.publicKey({
    message: "Authority:",
    default: umi.identity.publicKey,
    required: true,
  }),
  bondFeeBps: await prompt.amount({
    message: "Bond fee (%):",
    identifier: "%",
    decimals: 2,
    default: createAmount(0n, "%", 2),
    min: createAmount(0n, "%", 2),
    max: createAmount(10_000n, "%", 2),
    required: true,
  }),
  disputeWindow: await prompt.integer({
    message: "Dispute window (secs):",
    default: 24 * 60 * 60,
    min: 0,
    max: 0xffffffff,
    required: true,
  }),
  votingWindow: await prompt.integer({
    message: "Voting window (secs):",
    default: 24 * 60 * 60,
    min: 0,
    max: 0xffffffff,
    required: true,
  }),
  arbitrationWindow: await prompt.integer({
    message: "Arbitration window (secs):",
    default: 12 * 60 * 60,
    min: 0,
    max: 0xffffffff,
    required: true,
  }),
};

logger.newline();

//////////////////////////////////////////////////

const config = generateSigner(umi);

logger.log("Proceeding will create a config with the following parameters.");
logger.newline();
logger.entry("Config", config.publicKey);
logger.entry("Authority", args.authority);
logger.entry("Bond fee", displayAmount(args.bondFeeBps));
logger.entry("Dispute window", formatDuration(args.disputeWindow));
logger.entry("Voting window", formatDuration(args.votingWindow));
logger.entry("Arbitration window", formatDuration(args.arbitrationWindow));
logger.newline();

if (!(await prompt.confirm({ message: "Send transaction?" }))) {
  cancel();
}

logger.newline();

//////////////////////////////////////////////////

const builder = createConfigV1(umi, {
  config,
  authority: args.authority,
  bondFeeBps: Number(args.bondFeeBps.basisPoints),
  disputeWindow: args.disputeWindow,
  votingWindow: args.votingWindow,
  arbitrationWindow: args.arbitrationWindow,
});

const result = await spinner("Sending transaction...", builder.sendAndConfirm(umi));

const [signature] = base58.deserialize(result.signature);
const error = result.result.value.err;

logger.entry("Signature", signature);

if (error !== null) {
  logger.newline();
  logger.error(error);

  process.exit(1);
}

process.exit(0);
