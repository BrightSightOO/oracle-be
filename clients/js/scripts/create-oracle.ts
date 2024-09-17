import type { Mint } from "@metaplex-foundation/mpl-toolbox";
import type { PublicKey } from "@metaplex-foundation/umi";

import { safeFetchMint } from "@metaplex-foundation/mpl-toolbox";
import {
  amountToString,
  isZeroAmount,
  keypairIdentity,
  unwrapOption,
} from "@metaplex-foundation/umi";
import { base58 } from "@metaplex-foundation/umi/serializers";

import { createOracleV1 } from "../src";

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
    logger.error("Wallet balance is empty, are you using the correct wallet?");

    process.exit(1);
  }
}

//////////////////////////////////////////////////

type OracleArgs = {
  authority: PublicKey;
  governanceMint: PublicKey;
};

const args: OracleArgs = {
  authority: await prompt.publicKey({
    message: "Authority:",
    default: umi.identity.publicKey,
    required: true,
  }),
  governanceMint: await prompt.publicKey({
    message: "Governance Token:",
    required: true,
  }),
};

logger.newline();

//////////////////////////////////////////////////

let mint: Mint | null;
try {
  mint = await safeFetchMint(umi, args.governanceMint);
} catch (err) {
  if (!(err instanceof Error) || err.name !== "UnexpectedAccountError") {
    throw err;
  }

  logger.error(`Invalid governance token, account [${args.governanceMint}] is not a mint`);

  process.exit(1);
}

logger.log("Proceeding will create a config with the following parameters.");
logger.newline();
logger.entry("Authority", args.authority);
logger.entry("Governance Token", args.governanceMint);
logger.newline();
if (mint !== null) {
  logger.group("Governance Token", (group) => {
    group.entry("Mint Authority", unwrapOption(mint.mintAuthority) ?? "None");
    group.entry("Freeze Authority", unwrapOption(mint.freezeAuthority) ?? "None");
    group.entry("Decimals", mint.decimals);
  });
} else {
  logger.warn("Governance token mint doesn't exist");
}
logger.newline();

if (!(await prompt.confirm({ message: "Send transaction?" }))) {
  cancel();
}

logger.newline();

//////////////////////////////////////////////////

const builder = createOracleV1(umi, {
  authority: args.authority,
  governanceMint: args.governanceMint,
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
