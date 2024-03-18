//@ts-check

import path from "path";
import { fileURLToPath } from "url";

import { generateIdl } from "@metaplex-foundation/shank-js";

const __dirname = fileURLToPath(new URL(".", import.meta.url));

const rootDir = path.dirname(__dirname);

const idlDir = path.join(rootDir, "idls");
const programDir = path.join(rootDir, "programs");
const binaryInstallDir = path.join(rootDir, ".crates");

await generateIdl({
  generator: "shank",
  programName: "oracle",
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "oracle"),
});
