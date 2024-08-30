<h1 align="center">
  Optimistic Oracle
</h1>
<p align="center">
  A source of truth for on-chain data.
</p>

> [!WARNING]
> This project has not been formally audited – use in production at your own risk.

## How it works

1. A user creates a request, specifying:

   - the question to be answered
   - the assertion timestamp after which the question can be answered
   - the amount of a currency that must be bonded in order to assert an answer
     for the request
   - the reward for answering the request

2. After the assertion timestamp has been reached, another user can propose a
   value to answer the request question.

   The user must put up a bond in order to make an assertion.

3. After an assertion is made, there is a window of time in which the value may
   be disputed.

   In order to dispute an assertion the disputer must put up a bond equal to
   that the asserter bonded.

   - If the dispute window elapses without a dispute being made, then the
     assertion is treated as the truth and the request may be resolved.

     The asserter can then reclaim their bond and claim the request reward.

   - If the assertion is disputed, then there follows a voting period where
     users with a stake in the governance token may vote to submit the what they
     believe to be the correct answer.

     After the voting period elapses, then the request will resolve with the
     value that received the most votes.

     The correct party amongst the asserter and disputer may claim both bonds.

## Building

From the root directory of the repository:

- Install the required packages:

```sh
pnpm install
```

- Build the program:

```sh
pnpm programs:build
```

This will create program binaries in the `<ROOT>/.bin` directory.

## Programs

This project contains the following programs:

- [Optimistic Oracle](./programs/oracle) `DVM2j1a1AJ9hZuEXyMxA5vusKgMR2FcKJyCf3QE5R2ge`

You will need a Rust version compatible with BPF to compile the programs,
currently we recommend using Rust 1.75.0.

## Clients

This project contains the following clients:

- [JavaScript (UMI)](./clients/js)
- [Rust](./clients/rust)

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT License](LICENSE-MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
