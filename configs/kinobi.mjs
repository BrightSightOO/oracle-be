// @ts-check

import fs from "fs/promises";
import path from "path";
import { fileURLToPath } from "url";

import * as kIdl from "@kinobi-so/nodes-from-anchor";
import * as kJs from "@kinobi-so/renderers-js-umi";
import * as kRust from "@kinobi-so/renderers-rust";
import { bold } from "colorette";
import { ESLint } from "eslint";
import * as k from "kinobi";

const __dirname = fileURLToPath(new URL(".", import.meta.url));

const rootDir = path.dirname(__dirname);
const idlDir = path.join(rootDir, "idls");
const clientDir = path.join(rootDir, "clients");

const idlJson = await fs.readFile(path.join(idlDir, "optimistic_oracle.json"), "utf8");

const start = Date.now();

console.log("generating clients...");

const idl = kIdl.rootNodeFromAnchor(JSON.parse(idlJson));
const kinobi = k.createFromRoot(idl);

// Update accounts.
kinobi.update(
  k.updateAccountsVisitor({
    oracleV1: {
      seeds: [k.constantPdaSeedNodeFromString("utf8", "oracle")],
    },
    currencyV1: {
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "currency"),
        k.variablePdaSeedNode("config", k.publicKeyTypeNode(), "The address of the config."),
        k.variablePdaSeedNode("mint", k.publicKeyTypeNode(), "The address of the currency mint."),
      ],
    },
    requestV1: {
      size: null,
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "request"),
        k.variablePdaSeedNode(
          "index",
          k.numberTypeNode("u64"),
          "The next request index in the oracle.",
        ),
      ],
    },
    assertionV1: {
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "assertion"),
        k.variablePdaSeedNode("request", k.publicKeyTypeNode(), "The address of the request."),
      ],
    },
    stakeV1: {
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "stake"),
        k.variablePdaSeedNode("wallet", k.publicKeyTypeNode(), "The address of the wallet."),
      ],
    },
    votingV1: {
      size: null,
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "voting"),
        k.variablePdaSeedNode("request", k.publicKeyTypeNode(), "The address of the request."),
      ],
    },
    voteV1: {
      seeds: [
        k.constantPdaSeedNodeFromString("utf8", "vote"),
        k.variablePdaSeedNode(
          "voting",
          k.publicKeyTypeNode(),
          "The address of the voting account.",
        ),
        k.variablePdaSeedNode("stake", k.publicKeyTypeNode(), "The address of the stake account."),
      ],
    },
  }),
);

// Set default values for instruction accounts.
kinobi.update(
  k.setInstructionAccountDefaultValuesVisitor([
    {
      account: "oracle",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("oracleV1"),
    },
    {
      account: "currency",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("currencyV1", [
        k.pdaSeedValueNode("config", k.accountValueNode("config")),
        k.pdaSeedValueNode("mint", k.accountValueNode("mint")),
      ]),
    },
    {
      account: "assertion",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("assertionV1", [
        k.pdaSeedValueNode("request", k.accountValueNode("request")),
      ]),
    },
    {
      account: "voting",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("votingV1", [
        k.pdaSeedValueNode("request", k.accountValueNode("request")),
      ]),
    },
    {
      account: "vote",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("voteV1", [
        k.pdaSeedValueNode("voting", k.accountValueNode("voting")),
        k.pdaSeedValueNode("stake", k.accountValueNode("stake")),
      ]),
    },
  ]),
);

const ataPdaValueNode = (mint = "mint", owner = "owner") =>
  k.pdaValueNode(k.pdaLinkNode("associatedToken", "mplToolbox"), [
    k.pdaSeedValueNode("mint", k.accountValueNode(mint)),
    k.pdaSeedValueNode("owner", k.accountValueNode(owner)),
  ]);

// Update instructions.
kinobi.update(
  k.updateInstructionsVisitor({
    createRequestV1: {
      accounts: {
        bondCurrency: {
          defaultValue: k.pdaValueNode("currencyV1", [
            k.pdaSeedValueNode("config", k.accountValueNode("config")),
            k.pdaSeedValueNode("mint", k.argumentValueNode("bondMint")),
          ]),
        },
        rewardCurrency: {
          defaultValue: k.pdaValueNode("currencyV1", [
            k.pdaSeedValueNode("config", k.accountValueNode("config")),
            k.pdaSeedValueNode("mint", k.accountValueNode("rewardMint")),
          ]),
        },
        rewardSource: {
          defaultValue: ataPdaValueNode("rewardMint", "creator"),
        },
        rewardEscrow: {
          defaultValue: k.pdaValueNode(k.pdaLinkNode("reward", "hooked"), [
            k.pdaSeedValueNode("request", k.accountValueNode("request")),
          ]),
        },
        creator: {
          defaultValue: k.identityValueNode(),
        },
      },
      arguments: {
        bondMint: {
          docs: ["Bond mint"],
          type: k.publicKeyTypeNode(),
        },
      },
    },
    createAssertionV1: {
      accounts: {
        bondSource: {
          defaultValue: ataPdaValueNode("bondMint", "asserter"),
        },
        bondEscrow: {
          defaultValue: k.pdaValueNode(k.pdaLinkNode("assertBond", "hooked"), [
            k.pdaSeedValueNode("request", k.accountValueNode("request")),
          ]),
        },
        governanceSource: {
          defaultValue: ataPdaValueNode("governanceMint", "asserter"),
        },
        governanceEscrow: {
          defaultValue: k.pdaValueNode(k.pdaLinkNode("assertGovernanceBond", "hooked"), [
            k.pdaSeedValueNode("request", k.accountValueNode("request")),
          ]),
        },
        asserter: {
          defaultValue: k.identityValueNode(),
        },
      },
    },
  }),
);

// Mark timestamps fields as data-time types.
kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: (node) =>
        (node.kind === "structFieldTypeNode" || node.kind === "instructionArgumentNode") &&
        node.type.kind === "numberTypeNode" &&
        node.type.format === "i64" &&
        node.type.endian === "le" &&
        k.snakeCase(node.name).split("_").includes("timestamp"),
      transform: (node) => {
        k.assertIsNode(node, ["structFieldTypeNode", "instructionArgumentNode"]);
        k.assertIsNode(node.type, "numberTypeNode");

        return { ...node, type: k.dateTimeTypeNode(node.type) };
      },
    },
  ]),
);

/** @param {string} name */
const accountType = (name) => ({
  field: "accountType",
  value: k.enumValueNode("AccountType", name),
});

// Set account discriminators.
kinobi.update(
  k.setAccountDiscriminatorFromFieldVisitor({
    OracleV1: accountType("OracleV1"),
    ConfigV1: accountType("ConfigV1"),
    StakeV1: accountType("StakeV1"),
    RequestV1: accountType("RequestV1"),
    AssertionV1: accountType("AssertionV1"),
    CurrencyV1: accountType("CurrencyV1"),
    VotingV1: accountType("VotingV1"),
    VoteV1: accountType("VoteV1"),
  }),
);

// Render Rust.
{
  const crateDir = path.join(clientDir, "rust");
  const rustDir = path.join(crateDir, "src", "generated");

  console.log(`writing rust client to ${bold(path.relative(rootDir, rustDir))}...`);

  kinobi.accept(
    kRust.renderVisitor(rustDir, {
      crateFolder: crateDir,
      formatCode: true,
      toolchain: "+nightly",
    }),
  );
}

// Render JavaScript.
{
  const jsDir = path.join(clientDir, "js", "src", "generated");

  console.log(`writing js client to ${bold(path.relative(rootDir, jsDir))}...`);

  await kinobi.accept(
    kJs.renderVisitor(jsDir, {
      formatCode: true,
    }),
  );

  console.log("cleaning up generated js client code...");

  const eslint = new ESLint({
    cache: true,
    cacheLocation: path.join(rootDir, "node_modules", ".cache", "eslint-kinobi"),
    cacheStrategy: "content",
    fix: true,
  });
  const lintResults = await eslint.lintFiles(jsDir);

  await ESLint.outputFixes(lintResults);

  const eslintFormatter = await eslint.loadFormatter();
  const lintOutput = await eslintFormatter.format(lintResults);

  if (lintOutput) {
    console.error(lintOutput);
  }
}

console.log(`done in ${bold(`${Date.now() - start}ms`)}`);
