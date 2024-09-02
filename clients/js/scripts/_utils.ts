import type { Amount, Umi, UmiPlugin } from "@metaplex-foundation/umi";

import fs from "node:fs";

import { mplToolbox } from "@metaplex-foundation/mpl-toolbox";
import { keypairIdentity } from "@metaplex-foundation/umi";
import { createUmi as baseCreateUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { bold, red } from "colorette";

import { optimisticOracle } from "../src";

export const MAINNET_URL = "https://api.mainnet-beta.solana.com";

export function createUmi(rpcUrl: string): Umi {
  return baseCreateUmi(rpcUrl, { commitment: "confirmed" })
    .use(mplToolbox())
    .use(optimisticOracle());
}

export function walletKeypair(): UmiPlugin {
  const walletPath = process.env.WALLET_PATH;
  if (walletPath === undefined) {
    throw new Error("Missing WALLET_PATH environment variable");
  }

  let json: string;
  try {
    json = fs.readFileSync(walletPath, "utf8");
  } catch (err) {
    throw new Error(`Failed to read wallet file: ${walletPath}`, { cause: err });
  }

  let bytes: unknown;
  try {
    bytes = JSON.parse(json);
  } catch (err) {
    throw new Error("Failed to parse secret key", { cause: err });
  }

  if (!Array.isArray(bytes) || bytes.length !== 64) {
    throw new Error("Invalid secret key");
  }

  const secretKey = new Uint8Array(bytes);

  return {
    install(umi) {
      const keypair = umi.eddsa.createKeypairFromSecretKey(secretKey);

      umi.use(keypairIdentity(keypair));
    },
  };
}

export async function sleep(timeoutMs: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, timeoutMs));
}

export function createAmountFromDecimals<I extends string, D extends number>(
  decimalAmount: string,
  identifier: I,
  decimals: D,
): Amount<I, D> {
  const parts = decimalParts(decimalAmount);
  if (parts === undefined) {
    throw new Error(`Invalid decimal: ${decimalAmount}`);
  }

  const [intPart, decPart] = parts;

  let decPartInt = 0n;
  if (decPart !== undefined) {
    if (decPart.length > decimals) {
      throw new Error(`Too many decimals: ${decPart.length} > ${decimals}`);
    }
    decPartInt = BigInt(decPart.padEnd(decimals, "0"));
  }

  const factor = 10n ** BigInt(decimals);
  const basisPoints = BigInt(intPart) * factor + decPartInt;

  return { basisPoints, identifier, decimals };
}

export function decimalParts(value: string): [intPart: string, decPart?: string] | undefined {
  value = value.trim();

  const dotIdx = value.indexOf(".");

  if (dotIdx === -1) {
    if (!isDigits(value)) {
      return;
    }
    return [value];
  }

  const intPart = value.slice(0, dotIdx);
  const decPart = value.slice(dotIdx + 1);

  if (!isDigits(intPart) || !isDigits(decPart)) {
    return;
  }

  return [intPart, decPart];
}

export function isDigits(value: string): boolean {
  if (value.length === 0) {
    return false;
  }

  for (let i = 0; i < value.length; i++) {
    const code = value.charCodeAt(i);
    if (code < 0x30 || code > 0x39) {
      return false;
    }
  }

  return true;
}

export function formatDuration(secs: number): string {
  const d = (secs / 86_400) >>> 0;
  const h = ((secs % 86_400) / 3_600) >>> 0;
  const m = ((secs % 3_600) / 60) >>> 0;
  const s = secs % 60;

  let fmt = "";

  if (d > 0) {
    fmt += `${d}d`;
  }
  if (h > 0) {
    fmt += `${h}h`;
  }
  if (m > 0) {
    fmt += `${m}m`;
  }
  if (s > 0 || fmt.length === 0) {
    fmt += `${s}s`;
  }

  return fmt;
}

export function error(msg: string, exitCode = 1): never {
  console.error(`${bold(`${red("Error")}:`)} ${msg}`);

  process.exit(exitCode);
}
