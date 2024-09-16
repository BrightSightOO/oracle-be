import type { DeepPartial } from "../types";

import { bold, cyan, dim, green, isColorSupported, italic, red } from "colorette";

import * as spinner from "../spinner";

type DefaultTheme = {
  prefix: string;
  spinner: {
    interval: number;
    frames: Array<string>;
  };
  style: {
    answer: (text: string) => string;
    message: (text: string) => string;
    error: (text: string) => string;
    defaultAnswer: (text: string) => string;
    help: (text: string) => string;
    highlight: (text: string) => string;
    key: (text: string) => string;
  };
};

export type Theme<Extension extends Record<string, unknown> = Record<never, never>> = DefaultTheme &
  Extension;

export const defaultTheme: DefaultTheme = {
  prefix: green("?"),
  spinner: {
    interval: spinner.interval,
    frames: spinner.frames,
  },
  style: {
    answer: cyan,
    message: bold,
    error: (text: string): string => `> ${red(italic(text))}`,
    defaultAnswer: isColorSupported ? dim : (text: string) => `(${text})`,
    help: dim,
    highlight: cyan,
    key: (text: string): string => cyan(bold(`<${text}>`)),
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
