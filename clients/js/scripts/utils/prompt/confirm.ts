import type { Theme } from "./theme";
import type { DeepPartial } from "../types";
import type { Status } from "@inquirer/core";

import { cursorTo } from "node:readline";

import { createPrompt, isEnterKey, useKeypress, usePrefix, useState } from "@inquirer/core";

import { makeTheme } from "./theme";

type MaybePromise<T> = T | Promise<T>;

type PublicKeyConfig = {
  message: string;
  default?: boolean | undefined;
  transformer?: (value: boolean) => MaybePromise<string>;
  theme?: DeepPartial<Theme> | undefined;
};

export const confirm = createPrompt<boolean, PublicKeyConfig>((config, done) => {
  const { default: defaultValue = false, transformer = (v) => (v ? "yes" : "no") } = config;

  const [status, setStatus] = useState<Status>("idle");
  const [value, setValue] = useState<string>("");

  const theme = makeTheme(config.theme);
  const prefix = usePrefix({ status, theme });

  useKeypress(async (key, rl) => {
    // Ignore keypress while the prompt is not pending.
    if (status !== "idle") {
      return;
    }

    let result: boolean;

    if (isEnterKey(key)) {
      result = defaultValue;
    } else if (key.name === "y") {
      result = true;
    } else if (key.name === "n") {
      result = false;
    } else {
      rl.clearLine(0);

      // Fix bizarre issue where cursor position gets messed up.
      {
        const { cols: cursor } = rl.getCursorPos();
        rl.output.unmute();
        cursorTo(rl.output, cursor);
        rl.output.mute();
      }

      return;
    }

    setStatus("loading");
    setValue(await transformer(result));
    setStatus("done");
    done(result);
  });

  const message = theme.style.message(config.message, status);
  const defaultHint = theme.style.defaultAnswer(`(${defaultValue ? "Y/n" : "y/N"})`);
  const formattedValue = status === "done" ? theme.style.answer(value) : value;

  return `${prefix} ${message} ${defaultHint} ${formattedValue}`;
});

export default confirm;
