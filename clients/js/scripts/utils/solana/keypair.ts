import type { Keypair } from "@metaplex-foundation/umi";

import fs from "node:fs/promises";

import { publicKey } from "@metaplex-foundation/umi";

export class KeypairError extends Error {
  file: string;

  constructor(message: string, file: string, options?: ErrorOptions) {
    super(message, options);
    this.file = file;
  }
}

export async function readKeypair(file: string): Promise<Keypair> {
  let json: string;
  try {
    json = await fs.readFile(file, "utf8");
  } catch (err) {
    throw new KeypairError("Failed to read keypair", file, { cause: err });
  }

  let bytes: unknown;
  try {
    bytes = JSON.parse(json);
  } catch (err) {
    throw new KeypairError("Failed to parse keypair", file, { cause: err });
  }

  if (
    !Array.isArray(bytes) ||
    bytes.length !== 64 ||
    bytes.some((b) => typeof b !== "number" || b < 0 || b > 255)
  ) {
    throw new KeypairError("Invalid keypair", file);
  }

  const secretKey = new Uint8Array(bytes);
  const publicKeyBytes = secretKey.subarray(32);

  return { publicKey: publicKey(publicKeyBytes), secretKey };
}
