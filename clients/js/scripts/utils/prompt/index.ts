import { CancelPromptError, ExitPromptError } from "@inquirer/core";

export * from "./theme";

export { default as amount } from "./amount";
export { default as publicKey } from "./public-key";
export { default as integer } from "./integer";
export { default as confirm } from "./confirm";

export const isCancelError = (err: unknown): err is CancelPromptError | ExitPromptError =>
  err instanceof CancelPromptError || err instanceof ExitPromptError;
