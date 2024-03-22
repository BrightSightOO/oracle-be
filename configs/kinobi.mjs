// @ts-check

import { execFile } from "child_process";
import path from "path";
import { fileURLToPath } from "url";

import * as k from "@metaplex-foundation/kinobi";
import { bold } from "colorette";
import { ESLint } from "eslint";

const __dirname = fileURLToPath(new URL(".", import.meta.url));

const rootDir = path.dirname(__dirname);

const idlDir = path.join(rootDir, "idls");
const clientDir = path.join(rootDir, "clients");

const jsDir = path.join(clientDir, "js", "src", "generated");
const crateDir = path.join(clientDir, "rust");
const rustDir = path.join(crateDir, "src", "generated");

const start = Date.now();

console.log("generating client code...");

const kinobi = k.createFromIdls([path.join(idlDir, "oracle.json")]);

kinobi.update(
  k.updateProgramsVisitor({
    oracle: {
      name: "optimisticOracle",
    },
  }),
);

kinobi.update(k.defaultVisitor());

// Update accounts.
kinobi.update(
  k.updateAccountsVisitor({
    oracle: {
      seeds: [k.constantPdaSeedNodeFromString("oracle")],
    },
    request: {
      seeds: [
        k.constantPdaSeedNodeFromString("request"),
        k.variablePdaSeedNode(
          "index",
          k.numberTypeNode("u64"),
          "The next request index in the oracle.",
        ),
      ],
    },
    assertion: {
      seeds: [
        k.constantPdaSeedNodeFromString("assertion"),
        k.variablePdaSeedNode("request", k.publicKeyTypeNode(), "The address of the request."),
      ],
    },
  }),
);

const ataPdaDefault = (mint = "mint", owner = "owner") =>
  k.pdaValueNode(k.pdaLinkNode("associatedToken", "mplToolbox"), [
    k.pdaSeedValueNode("mint", k.accountValueNode(mint)),
    k.pdaSeedValueNode("owner", k.accountValueNode(owner)),
  ]);

// Set default values for instruction accounts.
kinobi.update(
  k.setInstructionAccountDefaultValuesVisitor([
    {
      account: "oracle",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("oracle"),
    },
    {
      account: "assertion",
      ignoreIfOptional: true,
      defaultValue: k.pdaValueNode("assertion", [
        k.pdaSeedValueNode("request", k.accountValueNode("request")),
      ]),
    },
  ]),
);

// Update instructions.
kinobi.update(
  k.updateInstructionsVisitor({
    createRequest: {
      accounts: {
        // TODO: Default rewardMint to SOL/USDC?
        rewardSource: {
          defaultValue: ataPdaDefault("rewardMint", "creator"),
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
    },
    createAssertion: {
      accounts: {
        // TODO: Default bondMint to SOL/USDC?
        bondSource: {
          defaultValue: ataPdaDefault("bondMint", "asserter"),
        },
        bondEscrow: {
          defaultValue: k.pdaValueNode(k.pdaLinkNode("assertBond", "hooked"), [
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

/** @param {string} name */
const accountType = (name) => ({
  field: "accountType",
  value: k.enumValueNode("AccountType", name),
});

// Set account discriminators.
kinobi.update(
  k.setAccountDiscriminatorFromFieldVisitor({
    Oracle: accountType("Oracle"),
    Stake: accountType("Stake"),
    Request: accountType("Request"),
    Assertion: accountType("Assertion"),
  }),
);

// Render Rust.
{
  console.log(`writing rust client to ${bold(path.relative(rootDir, rustDir))}...`);

  kinobi.accept(
    k.renderRustVisitor(rustDir, {
      formatCode: true,
      crateFolder: crateDir,
    }),
  );

  console.log("cleaning up generated rust client code...");

  execFile("cargo", ["fmt", `--manifest-path=${path.join(crateDir, "Cargo.toml")}`]);
}

// Render JavaScript.
{
  console.log(`writing js client to ${bold(path.relative(rootDir, jsDir))}...`);

  kinobi.accept(
    k.renderJavaScriptVisitor(jsDir, {
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
