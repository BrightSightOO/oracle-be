import type { Amount, PublicKey, UmiPlugin } from "@metaplex-foundation/umi";

import {
  amountToString,
  createAmount,
  createNoopSigner,
  displayAmount,
  isPublicKey,
  isZeroAmount,
  keypairIdentity,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { base58 } from "@metaplex-foundation/umi/serializers";
import { distance } from "fastest-levenshtein";

import { fetchConfigV1, getConfigV1GpaBuilder, updateConfigV1 } from "../src";

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
  ws: {
    type: "string",
    desc: "WebSocket URL for the solana cluster",
    valueName: "URL",
  },
  authority: {
    type: "string",
    alias: "k",
    desc: "Filepath to a keypair or public key of config authority",
  },
});

let { url: rpcUrl, authority } = argv;

let isKeypairIdentity = false;
let identityPlugin: UmiPlugin | undefined;

if (authority !== undefined && isPublicKey(authority)) {
  identityPlugin = signerIdentity(createNoopSigner(authority));
}

if (rpcUrl === undefined || authority === undefined) {
  const config = await readCliConfig(argv.config);

  rpcUrl ??= config.rpcUrl;
  authority ??= config.keypairPath;
}

if (identityPlugin === undefined) {
  const keypair = await readKeypair(authority);

  isKeypairIdentity = true;
  identityPlugin = keypairIdentity(keypair);
}

const umi = createUmi(rpcUrl, argv.ws).use(identityPlugin);

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

const configs = await getConfigV1GpaBuilder(umi)
  .whereField("authority", umi.identity.publicKey)
  .getPublicKeys();

if (configs.length === 0) {
  logger.bail(`Authority [${umi.identity.publicKey}] is not the authority of any configs.`);
}
configs.sort();

const configAddress = await prompt.search<PublicKey>({
  message: "Config:",
  source: (term) => {
    if (!term) {
      return configs;
    }

    type Distanced = [d: number, k: PublicKey];

    const termLower = term.toLowerCase();

    let filtered = configs.filter((k) => k.toLowerCase().startsWith(termLower));
    if (filtered.length === 0) {
      filtered = configs;
    }

    return configs
      .map<Distanced>((k) => [distance(term, k) + distance(termLower, k.toLowerCase()), k])
      .sort(([a], [b]) => a - b)
      .map(([, k]) => k);
  },
});

const config = await fetchConfigV1(umi, configAddress);

logger.newline();

//////////////////////////////////////////////////

const updateAuthority = await prompt.confirm({ message: "Update authority?" });

logger.newline();

type UpdateArgs =
  | { __kind: "Authority"; newAuthority: PublicKey }
  | {
      __kind: "Config";
      newBondFeeBps: Amount<"%", 2>;
      newDisputeWindow: number;
      newVotingWindow: number;
      newArbitrationWindow: number;
    };

const args: UpdateArgs = updateAuthority
  ? {
      __kind: "Authority",
      newAuthority: await prompt.publicKey({
        message: "New authority",
        default: config.authority,
        required: true,
      }),
    }
  : {
      __kind: "Config",
      newBondFeeBps: await prompt.amount({
        message: "New bond fee (%):",
        identifier: "%",
        decimals: 2,
        default: createAmount(config.bondFeeBps, "%", 2),
        min: createAmount(0n, "%", 2),
        max: createAmount(10_000n, "%", 2),
        required: true,
      }),
      newDisputeWindow: await prompt.integer({
        message: "New dispute window (secs):",
        default: config.disputeWindow,
        min: 0,
        max: 0xffffffff,
        required: true,
      }),
      newVotingWindow: await prompt.integer({
        message: "New voting window (secs):",
        default: config.votingWindow,
        min: 0,
        max: 0xffffffff,
        required: true,
      }),
      newArbitrationWindow: await prompt.integer({
        message: "New arbitration window (secs):",
        default: config.arbitrationWindow,
        min: 0,
        max: 0xffffffff,
        required: true,
      }),
    };

logger.newline();

//////////////////////////////////////////////////

const builder = updateConfigV1(umi, {
  config: configAddress,
  updateConfigV1Args:
    args.__kind === "Config"
      ? {
          ...args,
          newBondFeeBps: Number(args.newBondFeeBps.basisPoints),
        }
      : args,
});

if (!isKeypairIdentity) {
  const tx = await builder.buildAndSign(umi);
  const rawTx = umi.transactions.serialize(tx);

  const [encodedTx] = base58.deserialize(rawTx);

  logger.entry("Serialized Transaction", encodedTx);

  process.exit(0);
}

logger.log("Proceeding will create a config with the following parameters.");
logger.newline();
if (args.__kind === "Authority") {
  logger.entry("New authority", args.newAuthority);
} else {
  logger.entry("New bond fee", displayAmount(args.newBondFeeBps));
  logger.entry("New dispute window", formatDuration(args.newDisputeWindow));
  logger.entry("New voting window", formatDuration(args.newVotingWindow));
  logger.entry("New arbitration window", formatDuration(args.newArbitrationWindow));
}
logger.newline();

if (!(await prompt.confirm({ message: "Send transaction?" }))) {
  cancel();
}

logger.newline();

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
