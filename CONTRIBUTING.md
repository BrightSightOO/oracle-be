# Contributing to Optimistic Oracle

This is a quick guide to help you contribute to the Optimistic Oracle.

## Getting started

The root folder has a private `package.json` containing a few scripts and
JavaScript dependencies that help: generate IDLs, clients, and start a local
validator.

First, [ensure you have pnpm installed](https://pnpm.io/installation) and run
the following command to install the dependencies.

```sh
pnpm install
```

You will now have access to the following commands:

- `pnpm programs:build` - Build all programs.
- `pnpm programs:test` - Test all programs.
- `pnpm generate` - Shortcut for `pnpm generate:idls && pnpm generate:clients`.
- `pnpm generate:idls` - Generate IDLs for all programs, as configured in the [`configs/shank.mjs`](./configs/shank.mjs) file.
- `pnpm generate:clients` - Generate clients using Codama, as configured in the [`configs/codama.mjs`](./configs/codama.mjs) file.
- `pnpm validator` - Start a local validator using Amman, as configured in the [`configs/validator.cjs`](./configs/validator.cjs) file.

## Managing clients

Each client has its own README with instructions on how to get started. You can
find them in the [clients](./clients) directory.

- [JavaScript (Umi)](./clients/js)
- [Rust](./clients/rust)

In order the generate the clients, run the following command.

```sh
pnpm generate
```

You will need run `pnpm generate` to regenerate the clients when something
changes in the programs.

## Deploying programs

First build the program by running the following command.

```sh
pnpm programs:build
```

Then to generate a keypair for the program buffer account, run the following command.

```sh
solana-keygen new --no-bip39-passphrase --outfile buffer.json
```

Then to deploy using the buffer account, run the following command.

```sh
solana program deploy --use-rpc \
    --url "${RPC_URL}" \
    --keypair /path/to/authority-keypair.json \
    --program-id DVM3hK9SDgXLmVoLng1KrTJCzTnhw31hAnqTYP7uGCot \
    --buffer ./buffer.json \
    .bin/oracle_program.so
```

Once the program successfully deploys, to remove the buffer keypair file, run
the following command:

```sh
rm ./buffer.json
```
