import type { Amount, PublicKey } from "@metaplex-foundation/umi";

import { fetchMint } from "@metaplex-foundation/mpl-toolbox";
import {
  amountToString,
  createAmount,
  isZeroAmount,
  keypairIdentity,
  unwrapOption,
} from "@metaplex-foundation/umi";
import { base58 } from "@metaplex-foundation/umi/serializers";
import { distance } from "fastest-levenshtein";

import { bounds, createCurrencyV1, getConfigV1GpaBuilder } from "../src";

import {
  cancel,
  createUmi,
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

type CurrencyArgs = {
  config?: PublicKey;
  mint?: PublicKey;
  rewardMin?: Amount;
  rewardMax?: Amount;
  bondMin?: Amount;
  bondMax?: Amount;
};

const args: CurrencyArgs = {};

const authority = umi.identity;
const configs = await getConfigV1GpaBuilder(umi)
  .whereField("authority", authority.publicKey)
  .getPublicKeys();

if (configs.length === 0) {
  logger.bail(`Authority [${authority.publicKey}] is not the authority of any configs.`);
}
configs.sort();

args.config = await prompt.search<PublicKey>({
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

args.mint = await prompt.publicKey({
  message: "Currency Token:",
  required: true,
});

const mint = await fetchMint(umi, args.mint);

{
  const decimals = mint.decimals;
  const identifier = "splToken";

  const min = createAmount(1n, identifier, decimals);
  const max = createAmount(2n ** 64n - 1n, identifier, decimals);

  args.rewardMin = await prompt.amount({
    message: "Minimum Reward:",
    decimals,
    identifier,
    min,
    max,
    required: true,
  });
  args.rewardMax = await prompt.amount({
    message: "Maximum Reward:",
    decimals,
    identifier,
    min: args.rewardMin,
    max,
    default: max,
    required: true,
  });

  args.bondMin = await prompt.amount({
    message: "Minimum Bond:",
    decimals,
    identifier,
    min,
    max,
    required: true,
  });
  args.bondMax = await prompt.amount({
    message: "Maximum Bond:",
    decimals,
    identifier,
    min: args.bondMin,
    max,
    default: max,
    required: true,
  });
}

logger.newline();

//////////////////////////////////////////////////

logger.log("Proceeding will create a currency with the following parameters.");
logger.newline();
logger.entry("Config", args.config);
logger.entry("Mint", args.mint);
logger.entry(
  "Reward Range",
  `[${amountToString(args.rewardMin)}, ${amountToString(args.rewardMax)}]`,
);
logger.entry("Bond Range", `[${amountToString(args.bondMin)}, ${amountToString(args.bondMax)}]`);
logger.newline();
logger.group("Currency Token", (group) => {
  group.entry("Mint Authority", unwrapOption(mint.mintAuthority) ?? "None");
  group.entry("Freeze Authority", unwrapOption(mint.freezeAuthority) ?? "None");
  group.entry("Decimals", mint.decimals);
});
logger.newline();

if (!(await prompt.confirm({ message: "Send transaction?" }))) {
  cancel();
}

logger.newline();

//////////////////////////////////////////////////

const builder = createCurrencyV1(umi, {
  config: args.config,
  mint: args.mint,
  authority,
  rewardRange: bounds(args.rewardMin.basisPoints, args.rewardMax.basisPoints),
  bondRange: bounds(args.bondMin.basisPoints, args.bondMax.basisPoints),
});

const result = await spinner("Sending transaction...", builder.sendAndConfirm(umi));

const [signature] = base58.deserialize(result.signature);
const error = result.result.value.err;

logger.entry("Signature", signature);

if (error !== null) {
  logger.newline();
  logger.bail(error);
}

process.exit(0);
