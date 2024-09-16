export function normalizeRpcUrlIfMoniker(urlOrMoniker: string): string {
  switch (urlOrMoniker) {
    case "m":
    case "mainnet-beta":
      return "https://api.mainnet-beta.solana.com";
    case "t":
    case "testnet":
      return "https://api.testnet.solana.com";
    case "d":
    case "devnet":
      return "https://api.devnet.solana.com";
    case "l":
    case "localnet":
      return "http://localhost:8899";
    default:
      return urlOrMoniker;
  }
}

export function deriveWebsocketUrl(rpcUrl: string): string {
  const url = new URL(rpcUrl);
  const protocol = url.protocol === "http:" ? "ws:" : "wss:";
  const portString = url.port !== "" ? `:${Number(url.port) + 1}` : "";

  return `${protocol}//${url.hostname}${portString}`;
}
