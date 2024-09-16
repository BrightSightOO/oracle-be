import type { Theme } from "./theme";
import type { DeepPartial } from "../types";
import type { Status } from "@inquirer/core";
import type { CancelablePromise, Context } from "@inquirer/type";
import type { PublicKey, PublicKeyInput } from "@metaplex-foundation/umi";

import {
  createPrompt,
  isBackspaceKey,
  isEnterKey,
  useKeypress,
  useMemo,
  usePrefix,
  useState,
} from "@inquirer/core";
import { isPublicKey, publicKey as toPublicKey } from "@metaplex-foundation/umi";

import { makeTheme } from "./theme";

type MaybePromise<T> = T | Promise<T>;
type MaybeRequired<R, T> = R extends true ? T : T | undefined;

type PublicKeyConfig = {
  message: string;
  default?: PublicKeyInput | undefined;
  required?: boolean | undefined;
  validate?: (value: PublicKey | undefined) => MaybePromise<boolean | string>;
  theme?: DeepPartial<Theme> | undefined;
};

export const publicKey: <C extends PublicKeyConfig>(
  config: C,
  context?: Context,
) => CancelablePromise<MaybeRequired<C["required"], PublicKey>> = createPrompt((config, done) => {
  type Result = Parameters<typeof done>[0];

  const { required = false, validate } = config;

  const [status, setStatus] = useState<Status>("idle");
  const [value, setValue] = useState<string>("");

  const validDefault = useMemo(
    () => (config.default !== undefined ? toPublicKey(config.default) : undefined),
    [config.default],
  );
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
      const result = isPublicKey(input) ? input : undefined;

      let isValid: boolean | string = (!required && input === "") || result !== undefined;
      if (isValid) {
        if (validate) {
          isValid = await validate(result);
        }
      }

      if (isValid === true) {
        setValue(result ?? "");
        setStatus("done");
        done(result as unknown as Result);
      } else {
        // Reset readline value to the previous value. On line event, the value
        // get cleared, forcing the user to re-enter the value instead of fixing it.
        rl.clearLine(0);
        rl.write(value);
        setError(isValid || "Invalid public key");
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

      if (/^[123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz]*$/.test(rl.line)) {
        const input = rl.line;
        const result = isPublicKey(input) ? input : undefined;

        let isValid: boolean | string = (!required && input === "") || result !== undefined;
        if (isValid) {
          if (validate) {
            isValid = await validate(result);
          }
        }

        setValue(rl.line);
        setError(isValid !== true ? isValid || "Invalid public key" : undefined);
      } else {
        rl.write(null, { name: "backspace" });
      }

      setStatus("idle");
    }
  });

  const message = theme.style.message(config.message);

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

export default publicKey;
