import { CancelPromptError, ExitPromptError } from "@inquirer/core";

import { logger } from "./log";

export const installErrorHandler = (): void => {
  process.setUncaughtExceptionCaptureCallback((err) => {
    if (err instanceof CancelPromptError || err instanceof ExitPromptError) {
      cancel();
    }

    logger.newline();
    logger.error(err.message);
    logger.newline();
    console.log(err);

    process.exit(1);
  });
};

export function cancel(): never {
  logger.newline();
  logger.log("Cancelled.");

  process.exit(1);
}
