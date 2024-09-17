import type { Theme } from "./theme";
import type { DeepPartial } from "../types";
import type { Status } from "@inquirer/core";
import type { Context } from "@inquirer/type";

import {
  createPrompt,
  isBackspaceKey,
  isEnterKey,
  useKeypress,
  usePrefix,
  useState,
} from "@inquirer/core";

import { makeTheme } from "./theme";

type MaybePromise<T> = T | Promise<T>;

type IntType<C extends IntegerConfig> = "bigint" extends keyof C
  ? boolean extends C["bigint"]
    ? number | bigint
    : C["bigint"] extends true
      ? bigint
      : number
  : number;

type Result<C extends IntegerConfig> = C["required"] extends true
  ? IntType<C>
  : IntType<C> | undefined;

type IntegerConfig = {
  message: string;
  required?: boolean | undefined;
  theme?: DeepPartial<Theme> | undefined;
} & (
  | {
      default?: number | undefined;
      min?: number | undefined;
      max?: number | undefined;
      bigint?: false | undefined;
      validate?: (value: number | undefined) => MaybePromise<boolean | string>;
    }
  | {
      default?: number | bigint | undefined;
      min?: number | bigint | undefined;
      max?: number | bigint | undefined;
      bigint: true;
      validate?: (value: number | undefined) => MaybePromise<boolean | string>;
    }
  | {
      default?: number | bigint | undefined;
      min?: number | bigint | undefined;
      max?: number | bigint | undefined;
      bigint?: boolean | undefined;
      validate?: (value: number | bigint | undefined) => MaybePromise<boolean | string>;
    }
);

function toSafeInt(value: number | bigint, bigint: true): bigint;
function toSafeInt(value: number | bigint, bigint: false): number;
function toSafeInt(value: number | bigint, bigint: boolean): number | bigint;
function toSafeInt(value: number | bigint, bigint: boolean): number | bigint {
  if (bigint) {
    return BigInt(value);
  } else if (value < Number.MIN_SAFE_INTEGER) {
    return Number.MIN_SAFE_INTEGER;
  } else if (value > Number.MAX_SAFE_INTEGER) {
    return Number.MAX_SAFE_INTEGER;
  } else {
    return Math.floor(Number(value));
  }
}

function parseInt(input: string, bigint: true): bigint | undefined;
function parseInt(input: string, bigint: false): number | undefined;
function parseInt(input: string, bigint: boolean): number | bigint | undefined;
function parseInt(input: string, bigint: boolean): number | bigint | undefined {
  if (input === "") {
    return;
  }

  try {
    const value = BigInt(input);
    return bigint ? value : Number(value);
  } catch {
    return;
  }
}

function validateInt(
  value: number | bigint | undefined,
  config: Pick<IntegerConfig, "min" | "max">,
): boolean | string {
  if (value === undefined) {
    return false;
  }

  const { min, max } = config;

  if (typeof value === "number") {
    if (!Number.isInteger(value)) {
      return `Value must be an integer`;
    } else if (!Number.isSafeInteger(value)) {
      return `Value does not safely fit within number`;
    }
  }

  if (min !== undefined && value < min) {
    return `Value must be at least ${min}`;
  }

  if (max !== undefined && value > max) {
    return `Value must be at most ${max}`;
  }

  return true;
}

export const integer: <C extends IntegerConfig>(
  config: C,
  context?: Context,
) => Promise<Result<C>> = createPrompt((config, done) => {
  type Result = Parameters<typeof done>[0];

  const { bigint = false, required = false, validate } = config;

  const [status, setStatus] = useState<Status>("idle");
  const [value, setValue] = useState<string>("");

  const min = config.min !== undefined ? toSafeInt(config.min, bigint) : undefined;
  const max = config.max !== undefined ? toSafeInt(config.max, bigint) : undefined;

  const isInt = min === undefined || min < 0 ? /^[-\d]*$/ : /^\d*$/;

  const validDefault =
    config.default !== undefined && validateInt(config.default, { min, max }) === true
      ? config.default.toString()
      : undefined;
  const [defaultValue = "", setDefaultValue] = useState<string>(validDefault);
  const [error, setError] = useState<string>();

  const theme = makeTheme(config.theme);
  const prefix = usePrefix({ status, theme });

  useKeypress(async (key, rl) => {
    // Ignore keypress while the prompt is not idle.
    if (status !== "idle") {
      return;
    }

    if (isEnterKey(key)) {
      setStatus("loading");

      const input = value || defaultValue;
      const result = parseInt(input, bigint);

      let isValid: boolean | string = input === "";
      if (required || result !== undefined) {
        isValid = validateInt(result, { min, max });
      }
      if (isValid === true && validate) {
        // @ts-expect-error: TypeScript fails to infer the type of `validate` correctly.
        isValid = await validate(result);
      }

      if (isValid === true) {
        setValue(result?.toString() ?? "");
        setStatus("done");
        done(result as Result);
      } else {
        // Reset readline value to the previous value. On line event, the value
        // get cleared, forcing the user to re-enter the value instead of fixing it.
        rl.clearLine(0);
        rl.write(value);
        setError(isValid || "Invalid integer");
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

      if (isInt.test(rl.line)) {
        const input = rl.line;
        const result = parseInt(input, bigint);

        let isValid: boolean | string = input === "";
        if (required || result !== undefined) {
          isValid = validateInt(result, { min, max });
        }
        if (isValid === true && validate) {
          // @ts-expect-error: TypeScript fails to infer the type of `validate` correctly.
          isValid = await validate(result);
        }

        setValue(rl.line);
        setError(isValid !== true ? isValid || "Invalid integer" : undefined);
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

export default integer;
