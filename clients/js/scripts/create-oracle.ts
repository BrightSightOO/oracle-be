import type { Mint } from "@metaplex-foundation/mpl-toolbox";
import type { PublicKey } from "@metaplex-foundation/umi";

import { safeFetchMint } from "@metaplex-foundation/mpl-toolbox";
import {
  displayAmount,
  isPublicKey,
  isZeroAmount,
  transactionBuilder,
  unwrapOption,
} from "@metaplex-foundation/umi";
import { base58 } from "@metaplex-foundation/umi/serializers";
import { bold, yellow } from "colorette";
import prompts from "prompts";

import { createOracleV1 } from "../src";

import { MAINNET_URL, createUmi, error, walletKeypair } from "./_utils";

const rpcUrl = process.env.RPC_URL ?? MAINNET_URL;
const umi = createUmi(rpcUrl).use(walletKeypair());

console.log(`${bold("Cluster:")} ${umi.rpc.getCluster()}`);
console.log(`${bold("Endpoint:")} ${umi.rpc.getEndpoint()}`);

{
  const wallet = umi.identity.publicKey;
  const balance = await umi.rpc.getBalance(wallet);

  console.log();
  console.log(bold("Wallet"));
  console.log(`  ${bold("Address:")} ${wallet}`);
  console.log(`  ${bold("Balance:")} ${displayAmount(balance)}`);

  if (isZeroAmount(balance)) {
    console.log();
    console.log("Wallet balance is empty, are you using the correct wallet?");

    process.exit(1);
  }
}

//////////////////////////////////////////////////

type OracleArgs = {
  authority?: PublicKey | undefined;
  governanceMint?: PublicKey | undefined;
};

console.log();

const args: OracleArgs = await prompts([
  {
    type: "text",
    name: "authority",
    message: "Authority",
    initial: umi.identity.publicKey,

    format: (value: string) => (value === "" ? null : value),

    validate: (value: string) =>
      value === "" || isPublicKey(value) || "Invalid public key (leave blank to use default)",
  },
  {
    type: "text",
    name: "governanceMint",
    message: "Governance Token",

    format: (value: string) => (value === "" ? null : value),

    validate: (value: string) => isPublicKey(value) || "Invalid public key",
  },
]);

console.log();

if (args.authority === undefined || args.governanceMint === undefined) {
  console.log("Cancelled.");

  process.exit(1);
}

let mint: Mint | null;
try {
  mint = await safeFetchMint(umi, args.governanceMint);
} catch (err) {
  if (!(err instanceof Error) || err.name !== "UnexpectedAccountError") {
    throw err;
  }

  error(`The governance token [${args.governanceMint}] is not a mint`);
}

console.log("Proceeding will create oracle with the following parameters.");
console.log();
console.log(`${bold("Authority:")} ${args.authority}`);
console.log(`${bold("Governance Token:")} ${args.governanceMint}`);
console.log();

if (mint !== null) {
  console.log(bold("Governance Token"));
  console.log(`  ${bold("Mint Authority:")} ${unwrapOption(mint.mintAuthority) ?? "None"}`);
  console.log(`  ${bold("Freeze Authority:")} ${unwrapOption(mint.freezeAuthority) ?? "None"}`);
  console.log(`  ${bold("Decimals:")} ${mint.decimals}`);
} else {
  console.log(yellow("Governance token mint account doesn't exist"));
}
console.log();

type ConfirmSend = {
  send?: boolean | undefined;
};

const confirm: ConfirmSend = await prompts({
  type: "confirm",
  name: "send",
  message: "Send transaction?",
  initial: false,
});

if (confirm.send !== true) {
  console.log();
  console.log("Cancelled.");

  process.exit(1);
}

const builder = transactionBuilder().append(
  createOracleV1(umi, {
    authority: args.authority,
    governanceMint: args.governanceMint,
  }),
);

console.log();
console.log("Sending transaction...");

const { signature: signatureBytes, result } = await builder.sendAndConfirm(umi);

const [signature] = base58.deserialize(signatureBytes);

console.log();
console.log(`${bold("Signature:")} ${signature}`);

if (result.value.err !== null) {
  error(JSON.stringify(result.value.err));
}

process.exit(0);
