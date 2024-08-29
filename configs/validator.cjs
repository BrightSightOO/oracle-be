//@ts-check

const path = require("path");

const rootDir = path.dirname(__dirname);
const binDir = path.join(rootDir, ".bin");

/**
 * @param {string} binary
 * @returns {string}
 */
function getProgram(binary) {
  return path.join(binDir, binary);
}

/** @type {import("@metaplex-foundation/amman").AmmanConfig} */
module.exports = {
  validator: {
    matchFeatures: "mainnet-beta",
    commitment: "processed",
    accountsCluster: "https://api.mainnet-beta.solana.com/",
    programs: [
      {
        label: "Optimistic Oracle",
        programId: "DVM2j1a1AJ9hZuEXyMxA5vusKgMR2FcKJyCf3QE5R2ge",
        deployPath: getProgram("oracle_program.so"),
      },
    ],
  },
};
