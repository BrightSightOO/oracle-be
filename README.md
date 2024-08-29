# Optimistic Oracle

Program ID: `DVMcc2M87dcTZfp4PLvzGU1Aps4xrNmNpkHKEpWm3hJW`

The optimistic oracle serves as a source of truth for on-chain data.

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

## Developers

### Building

To build the program, run the command:

```
$ pnpm programs:build
```

The resulting binary will be in the `.bin` directory.

### Testing

To test the program, run the command:

```
$ pnpm programs:test
```

### Generating clients

To update the generated client code, run the command:

```
$ pnpm generate
```

### Local validator

To run a local test validator with the program, first build the program, then
run the command:

```
$ pnpm validator
```
