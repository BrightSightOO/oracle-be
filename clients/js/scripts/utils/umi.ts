import type { Umi } from "@metaplex-foundation/umi";

import { mplToolbox } from "@metaplex-foundation/mpl-toolbox";
import { createUmi as baseCreateUmi } from "@metaplex-foundation/umi-bundle-defaults";

import { optimisticOracle } from "../../src";

import { deriveWebsocketUrl, normalizeRpcUrlIfMoniker } from "./solana";

export function createUmi(rpcUrl: string, wsUrl?: string): Umi {
  rpcUrl = normalizeRpcUrlIfMoniker(rpcUrl);

  return baseCreateUmi(rpcUrl, {
    commitment: "confirmed",
    wsEndpoint: wsUrl ?? deriveWebsocketUrl(rpcUrl),
  })
    .use(mplToolbox())
    .use(optimisticOracle());
}
