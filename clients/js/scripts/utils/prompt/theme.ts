import type { DeepPartial } from "../types";
import type { Status } from "@inquirer/core";

import { blue, bold, cyan, dim, green, isColorSupported, italic, red } from "colorette";

import * as spinner from "../spinner";

type DefaultTheme = {
  /**
   * Prefix to prepend to the message. If a function is provided, it will be
   * called with the current status of the prompt, and the return value will be
   * used as the prefix.
   *
   * @remarks
   * If `status === 'loading'`, this property is ignored and the spinner (styled
   * by the `spinner` property) will be displayed instead.
   */
  prefix: string | Omit<Record<Status, string>, "loading">;

  /**
   * Configuration for the spinner that is displayed when the prompt is in the
   * `'loading'` state.
   *
   * We recommend the use of {@link https://github.com/sindresorhus/cli-spinners|cli-spinners} for a list of available spinners.
   */
  spinner: {
    /**
     * The time interval between frames, in milliseconds.
     */
    interval: number;

    /**
     * A list of frames to show for the spinner.
     */
    frames: Array<string>;
  };
  /**
   * Object containing functions to style different parts of the prompt.
   */
  style: {
    /**
     * Style to apply to the user's answer once it has been submitted.
     *
     * @param text - The user's answer.
     * @returns The styled answer.
     */
    answer: (text: string) => string;

    /**
     * Style to apply to the message displayed to the user.
     *
     * @param text - The message to style.
     * @param status - The current status of the prompt.
     * @returns The styled message.
     */
    message: (text: string, status: Status) => string;

    /**
     * Style to apply to error messages.
     *
     * @param text - The error message.
     * @returns The styled error message.
     */
    error: (text: string) => string;

    /**
     * Style to apply to the default answer when one is provided.
     *
     * @param text - The default answer.
     * @returns The styled default answer.
     */
    defaultAnswer: (text: string) => string;

    /**
     * Style to apply to help text.
     *
     * @param text - The help text.
     * @returns The styled help text.
     */
    help: (text: string) => string;

    /**
     * Style to apply to highlighted text.
     *
     * @param text - The text to highlight.
     * @returns The highlighted text.
     */
    highlight: (text: string) => string;

    /**
     * Style to apply to keyboard keys referred to in help texts.
     *
     * @param text - The key to style.
     * @returns The styled key.
     */
    key: (text: string) => string;
  };
};

export type Theme<Extension extends Record<string, unknown> = Record<never, never>> = DefaultTheme &
  Extension;

export const defaultTheme: DefaultTheme = {
  prefix: {
    idle: blue("?"),
    done: green("✔"),
  },
  spinner: {
    interval: spinner.interval,
    frames: spinner.frames,
  },
  style: {
    answer: cyan,
    message: bold,
    error: (text) => `! ${red(italic(text))}`,
    defaultAnswer: isColorSupported ? dim : (text) => `(${text})`,
    help: dim,
    highlight: cyan,
    key: (text) => cyan(bold(`<${text}>`)),
  },
};

export function makeTheme<Extension extends Record<string, unknown>>(
  ...themes: ReadonlyArray<DeepPartial<Theme<Extension>> | undefined>
): Theme<Extension> {
  return deepMerge(
    defaultTheme as DeepPartial<Theme<Extension>>,
    ...themes.filter((v) => v !== undefined),
  );
}

function isPlainObject(value: unknown): value is Record<string, unknown> {
  if (
    typeof value !== "object" ||
    value === null ||
    Object.prototype.toString.call(value) !== "[object Object]"
  ) {
    return false;
  }

  if (Object.getPrototypeOf(value) === null) {
    return true;
  }

  let proto: unknown = value;
  while (Object.getPrototypeOf(proto) !== null) {
    proto = Object.getPrototypeOf(proto);
  }
  return Object.getPrototypeOf(value) === proto;
}

function deepMerge<T extends Record<string, unknown>>(...objects: Array<DeepPartial<T>>): T {
  const result: Record<string, unknown> = {};

  for (const obj of objects) {
    for (const [key, value] of Object.entries(obj)) {
      const prev = result[key];

      result[key] = isPlainObject(value) && isPlainObject(prev) ? deepMerge(prev, value) : value;
    }
  }

  return result as T;
}
