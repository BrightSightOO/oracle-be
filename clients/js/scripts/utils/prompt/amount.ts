import type { Theme } from "./theme";
import type { DeepPartial } from "../types";
import type { Status } from "@inquirer/core";
import type { CancelablePromise, Context } from "@inquirer/type";
import type { Amount } from "@metaplex-foundation/umi";

import {
  createPrompt,
  isBackspaceKey,
  isEnterKey,
  useKeypress,
  usePrefix,
  useState,
} from "@inquirer/core";
import { amountToString } from "@metaplex-foundation/umi";

import { convertAmount, parseAmount, scaleBasisPoints } from "../amount";

import { makeTheme } from "./theme";

type MaybePromise<T> = T | Promise<T>;

type Result<
  C extends AmountConfig<I, D>,
  I extends string = C["identifier"],
  D extends number = C["decimals"],
> = C["required"] extends true ? Amount<I, D> : Amount<I, D> | undefined;

type AmountConfig<I extends string = string, D extends number = number> = {
  message: string;
  identifier: I;
  decimals: D;
  default?: Amount<I, D> | undefined;
  min?: Amount<I, D> | undefined;
  max?: Amount<I, D> | undefined;
  required?: boolean | undefined;
  validate?: (value: Amount<I, D> | undefined) => MaybePromise<boolean | string>;
  theme?: DeepPartial<Theme> | undefined;
};

function validateAmount<I extends string, D extends number>(
  amount: Amount<I> | undefined,
  config: Pick<AmountConfig<I, D>, "decimals" | "min" | "max">,
): boolean | string {
  if (amount === undefined) {
    return false;
  }

  if (amount.decimals > config.decimals) {
    return `Value must have at most ${config.decimals} decimals`;
  }

  const basisPoints = scaleBasisPoints(amount.basisPoints, amount.decimals, config.decimals);

  if (config.min) {
    const min = convertAmount(config.min, config.decimals);

    if (basisPoints < min.basisPoints) {
      return `Value must be at least ${amountToString(min)}`;
    }
  }

  if (config.max) {
    const max = convertAmount(config.max, config.decimals);

    if (basisPoints > max.basisPoints) {
      return `Value must be at most ${amountToString(max)}`;
    }
  }

  return true;
}

export const amount: <I extends string, D extends number, C extends AmountConfig<I, D>>(
  config: C,
  context?: Context,
) => CancelablePromise<Result<C, I, D>> = createPrompt((config, done) => {
  type Result = Parameters<typeof done>[0];

  const { identifier, decimals, required = false, validate } = config;

  const [status, setStatus] = useState<Status>("idle");
  const [value, setValue] = useState<string>("");

  const min = config.min ? convertAmount(config.min, decimals, identifier) : undefined;
  const max = config.max ? convertAmount(config.max, decimals, identifier) : undefined;

  const validDefault =
    config.default && validateAmount(config.default, { decimals, min, max }) === true
      ? amountToString(config.default, decimals)
      : undefined;
  const [defaultValue = "", setDefaultValue] = useState<string>(validDefault);
  const [error, setError] = useState<string>();

  const theme = makeTheme(config.theme);
  const prefix = usePrefix({ status, theme });

  useKeypress(async (key, rl) => {
    // Ignore keypress while the prompt is not pending.
    if (status !== "idle") {
      return;
    }

    if (isEnterKey(key)) {
      setStatus("loading");

      const input = value || defaultValue;
      const parsedAmount = parseAmount(input, identifier);

      let result: Result | undefined;
      let isValid: boolean | string = input === "";
      if (required || parsedAmount !== undefined) {
        isValid = validateAmount(parsedAmount, { decimals, min, max });
      }
      if (isValid === true) {
        if (parsedAmount !== undefined) {
          result = convertAmount(parsedAmount, decimals);
        }
        if (validate) {
          isValid = await validate(result);
        }
      }

      if (isValid === true) {
        setValue(result ? amountToString(result) : "");
        setStatus("done");
        done(result as unknown as Result);
      } else {
        // Reset readline value to the previous value. On line event, the value
        // get cleared, forcing the user to re-enter the value instead of fixing it.
        rl.clearLine(0);
        rl.write(value);
        setError(isValid || "Invalid value");
        setStatus("idle");
      }
    } else if (isBackspaceKey(key) && !value) {
      // Clear the default value.
      setDefaultValue(undefined);
    } else if (key.name === "tab" && !value) {
      // Fill input with the default value.
      setDefaultValue(undefined);
      rl.clearLine(0); // Remove the tab character.
      rl.write(defaultValue);
      setValue(defaultValue);
    } else {
      setStatus("loading");

      if (/^[\d.]*$/.test(rl.line)) {
        const input = rl.line;
        const parsedAmount = parseAmount(input, identifier);

        let result: Result;
        let isValid: boolean | string = input === "";
        if (required || parsedAmount !== undefined) {
          isValid = validateAmount(parsedAmount, { decimals, min, max });
        }
        if (isValid === true) {
          if (parsedAmount !== undefined) {
            result = convertAmount(parsedAmount, decimals);
          }
          if (validate) {
            isValid = await validate(result);
          }
        }

        setValue(rl.line);
        setError(isValid !== true ? isValid || "Invalid value" : undefined);
      } else {
        rl.write(null, { name: "backspace" });
      }

      setStatus("idle");
    }
  });

  const message = theme.style.message(config.message, status);

  let displayValue = value;
  if (status === "done") {
    displayValue = theme.style.answer(value);
  } else if (defaultValue && !value) {
    displayValue = theme.style.defaultAnswer(defaultValue);
  }

  let formattedError = "";
  if (error) {
    formattedError = theme.style.error(error);
  }

  return [`${prefix} ${message} ${displayValue}`, formattedError];
});

export default amount;
