import { inspect } from "node:util";

import { bold, red, yellow } from "colorette";

type GroupCallback = (group: Omit<Logger, "error" | "bail">) => void;

type Logger = {
  newline(): void;
  log(message: unknown): void;
  warn(message: unknown): void;
  error(message: unknown): void;
  bail(message: unknown, exitCode?: number): never;
  entry(label: string, message: unknown): void;
  group(callback: GroupCallback): void;
  group(label: string, callback: GroupCallback): void;
};

function stringify(message: unknown): string {
  switch (typeof message) {
    case "function":
    case "object":
      return inspect(message, {
        colors: false,
        compact: true,
        depth: null,
      });

    case "string":
      return message;

    default:
      return String(message);
  }
}

export const logger: Logger = {
  newline() {
    console.log();
  },
  log(message) {
    console.log(stringify(message));
  },
  warn(message) {
    console.log(yellow(stringify(message)));
  },
  error(message) {
    console.log(`${bold(`${red("error")}:`)} ${stringify(message)}`);
  },
  bail(message, exitCode = 1) {
    this.error(message);

    process.exit(exitCode);
  },
  entry(label, message) {
    console.log(`${bold(`${label}:`)} ${stringify(message)}`);
  },
  group(...args: [label: string, callback: GroupCallback] | [callback: GroupCallback]) {
    let label: string | undefined;
    let callback: GroupCallback;

    if (args.length === 1) {
      [callback] = args;
    } else {
      [label, callback] = args;
    }

    if (label !== undefined) {
      console.group(bold(label));
    } else {
      console.group();
    }

    try {
      callback(this);
    } finally {
      console.groupEnd();
    }
  },
};
