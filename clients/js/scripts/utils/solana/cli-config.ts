import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { YAML } from "zx";

export type CliConfig = {
  rpcUrl: string;
  wsUrl: string | undefined;
  keypairPath: string;
};

export class CliConfigError extends Error {
  file: string;

  constructor(message: string, file: string) {
    super(message);
    this.file = file;
  }
}

export function defaultCliConfigPath(): string {
  return path.join(os.homedir(), ".config", "solana", "cli", "config.yml");
}

export async function readCliConfig(file: string = defaultCliConfigPath()): Promise<CliConfig> {
  const yaml = await fs.readFile(file, "utf8");
  const config: unknown = YAML.parse(yaml);

  if (typeof config !== "object" || config === null) {
    throw new CliConfigError("Invalid Solana CLI config", file);
  }

  if (!("json_rpc_url" in config) || typeof config.json_rpc_url !== "string") {
    throw new CliConfigError("Invalid 'json_rpc_url' in Solana CLI config", file);
  }
  if (!("websocket_url" in config) || typeof config.websocket_url !== "string") {
    throw new CliConfigError("Invalid 'websocket_url' in Solana CLI config", file);
  }
  if (!("keypair_path" in config) || typeof config.keypair_path !== "string") {
    throw new CliConfigError("Invalid 'keypair_path' in Solana CLI config", file);
  }

  const { json_rpc_url: rpcUrl, websocket_url: wsUrl, keypair_path: keypairPath } = config;

  return { rpcUrl, wsUrl: wsUrl !== "" ? wsUrl : undefined, keypairPath };
}
