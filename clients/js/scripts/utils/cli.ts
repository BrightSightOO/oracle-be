import path from "node:path";

import { bold, cyan, green, red, yellow } from "colorette";
import stringWidth from "string-width";
import { minimist } from "zx";

import { logger } from "./log";

type Arg = {
  name: string;
  required?: boolean | undefined;
};
type Args = Array<Arg>;

type ExpandArgs<Expanded extends Array<string | undefined>, Rest extends Args> = Rest extends []
  ? Expanded
  : Rest extends [infer Head extends Arg, ...infer Tail extends Args]
    ? ExpandArgs<[...Expanded, Head extends { required: true } ? string : string | undefined], Tail>
    : [...Expanded, ...Array<string>];

type Option = {
  alias?: string | undefined;
  desc?: string | undefined;
} & (
  | {
      type: "string";
      default?: string | undefined;
      valueName?: string | undefined;
    }
  | {
      type: "number";
      default?: number | undefined;
      valueName?: string | undefined;
    }
  | {
      type: "boolean";
      default?: boolean | undefined;
    }
);

type Options = Record<string, Option>;
type OptionValue = Option["default"];

type ParsedOption<O extends Option> = O extends { type: "string" }
  ? O extends { default: string }
    ? string
    : string | undefined
  : O extends { type: "number" }
    ? O extends { default: number }
      ? number
      : number | undefined
    : O extends { type: "boolean" }
      ? O extends { default: boolean }
        ? boolean
        : boolean | undefined
      : never;

type ParsedArgs<A extends Args> = { _: ExpandArgs<[], A> };
type ParsedOptions<O extends Options> = { [K in keyof O]: ParsedOption<O[K]> };

type CliArgs<O extends Options = Options, A extends Args = Args> = ParsedArgs<A> & ParsedOptions<O>;

export function parseCliArgs<O extends Options>(options: O, args?: undefined): CliArgs<O, []>;
export function parseCliArgs<O extends Options, A extends Args>(
  options: O,
  args: [...A],
): CliArgs<O, A>;

export function parseCliArgs(options: Options, args: Args = []): CliArgs {
  const opts: Array<string> = [];
  const flags: Array<string> = [];
  const aliases: Record<string, string | Array<string>> = {};
  const defaults: Record<string, unknown> = {};

  let required = 0;

  if (args.length > 0) {
    let optional = 0;

    for (const arg of args) {
      if (arg.required) {
        required++;

        if (optional > 0) {
          throw new Error("Cannot have required args after optional args");
        }
      } else {
        optional++;
      }
    }
  }

  if (options._ !== undefined) {
    throw new Error("Illegal option name: '_'");
  }

  const optionsEntries = Object.entries<Option>(options);

  if (options.help === undefined) {
    optionsEntries.push([
      "help",
      {
        type: "boolean",
        alias: "h",
        desc: "Print help",
      },
    ]);
  }

  for (const [name, opt] of optionsEntries) {
    switch (opt.type) {
      case "boolean":
        flags.push(name);
        break;
      case "number":
      case "string":
        opts.push(name);
        break;
    }

    if (opt.alias !== undefined) {
      aliases[name] = opt.alias;
    }

    if (opt.default !== undefined) {
      defaults[name] = opt.default;
    }
  }

  const parsed = minimist(process.argv.slice(2), {
    string: opts,
    boolean: flags,
    alias: aliases,
    default: defaults,
    unknown: (arg) => {
      if (arg.startsWith("-")) {
        logger.error(`Unexpected argument '${yellow(bold(arg))}' found\n`);

        printHelpAndExit(args, optionsEntries, 1);
      }
      return true;
    },
  });

  if (options.help === undefined && parsed.help === true) {
    printHelpAndExit(args, optionsEntries, 0);
  }

  if (parsed._.length < required) {
    const missing = args
      .slice(parsed._.length, parsed._.length + required)
      .map((v) => red(bold(`    <${v.name.toUpperCase()}>`)))
      .join("\n");

    logger.error(`The following required arguments were not provided:\n${missing}\n`);

    printHelpAndExit(args, optionsEntries, 1);
  }

  const result: Record<string, NonNullable<OptionValue> | Array<string>> = { _: parsed._ };

  for (const [name, opt] of optionsEntries) {
    let value = parsed[name] as OptionValue;
    if (value === undefined) {
      continue;
    }

    switch (opt.type) {
      case "string":
        value = String(value);
        break;
      case "number":
        value = Number(value);
        break;
      case "boolean":
        value = Boolean(value);
        break;
    }

    result[name] = value;
  }

  return result as CliArgs;
}

function printHelpAndExit(
  args: Array<Arg>,
  options: Array<[name: string, option: Option]>,
  exitCode = 1,
): never {
  const script = getScriptName();

  const columns: Array<{ opts: string; optsLen: number; desc: string }> = [];

  let maxOptsLen = 0;

  for (const [name, option] of options) {
    let short: string | undefined;
    let long: string | undefined;

    if (name.length === 1) {
      short = name;
    } else {
      long = name;
      if (option.alias?.length === 1) {
        short = option.alias;
      }
    }

    let opts = short ? bold(`-${short}`) : "  ";
    if (long) {
      opts += `${short ? ", " : "  "}${bold(`--${long}`)}`;

      if (option.type !== "boolean") {
        opts += ` <${option.valueName ?? long.toUpperCase()}>`;
      }
    }
    opts = cyan(opts);

    // Keep track of longest opts.
    const optsLen = stringWidth(opts);
    if (optsLen > maxOptsLen) {
      maxOptsLen = optsLen;
    }

    columns.push({ opts, optsLen, desc: option.desc ?? "" });
  }

  // Determine how much space each description line can take up.
  const descPad = 4 + maxOptsLen;
  const descSpace = Math.max(1, Math.floor(process.stdout.columns * 0.75) - descPad - 1);

  const usage = [green(bold("Usage:")), cyan(bold(script))];
  if (options.length > 0) {
    usage.push(cyan("[OPTIONS]"));
  }
  for (const arg of args) {
    usage.push(cyan(arg.required ? `<${arg.name.toUpperCase()}>` : `[${arg.name.toUpperCase()}]`));
  }

  let help = `${usage.join(" ")}\n\n${green(bold("Options:"))}`;

  // Build help lines.
  for (const { opts, optsLen, desc } of columns) {
    const optsPad = " ".repeat(maxOptsLen - optsLen);
    const wrappedDesc = wrap(desc, descSpace).replaceAll("\n", `\n${" ".repeat(descPad)}`);

    help += `\n  ${opts}${optsPad}  ${wrappedDesc}`;
  }

  // Extra blank line.
  help += "\n";

  logger.log(help);

  process.exit(exitCode);
}

function wrap(text: string, hardWidth: number): string {
  let result = "";

  let start = 0;
  let lineWidth: number;
  let carryoverWhitespace: string | undefined;

  while (start < text.length) {
    lineWidth = 0;

    const newline = text.indexOf("\n", start);
    const end = newline === -1 ? text.length : newline + 1;

    const line = text.slice(start, end);
    const words = findWords(line);

    if (carryoverWhitespace !== undefined) {
      const firstWord = words[0];
      if (firstWord !== undefined) {
        carryoverWhitespace = firstWord.trim() === "" ? firstWord : "";
      }
    }

    let i = 0;
    while (i < words.length) {
      const word = words[i]!;
      const trimmed = word.trimEnd();
      const wordWidth = stringWidth(trimmed);
      const trimmedDelta = word.length - trimmed.length;

      if (i !== 0 && hardWidth < lineWidth + wordWidth) {
        if (i > 0) {
          const last = i - 1;
          const trimmed = words[last]!.trimEnd();
          words[last] = trimmed;
        }

        lineWidth = 0;
        words.splice(i, 0, "\n");
        i++;

        if (carryoverWhitespace !== undefined) {
          lineWidth += carryoverWhitespace.length;
          words.splice(i, 0, carryoverWhitespace);
          i++;
        }
      }
      lineWidth += wordWidth + trimmedDelta;

      i++;
    }

    result += words.join("");

    start = end;
  }

  return result;
}

function findWords(line: string): Array<string> {
  const words: Array<string> = [];

  let start = 0;
  let inWhitespace = false;

  for (let i = 0; i < line.length; i++) {
    const nextWhitespace = line.codePointAt(i) === 0x20;
    if (inWhitespace && !nextWhitespace) {
      words.push(line.slice(start, i));

      start = i;
      inWhitespace = nextWhitespace;

      continue;
    }

    inWhitespace = nextWhitespace;
  }

  if (start < line.length) {
    words.push(line.slice(start));
  }

  return words;
}

export function getScriptName(): string {
  const { npm_lifecycle_env } = process.env;

  // Name of script in "package.json".
  if (npm_lifecycle_env) {
    return npm_lifecycle_env;
  }

  const scriptPath = process.argv[1];
  if (!scriptPath) {
    return "script";
  }

  const script = path.basename(scriptPath);
  const ext = path.extname(script);

  return script.endsWith(ext) ? script.slice(0, script.length - ext.length) : script;
}
