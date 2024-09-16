import { clearLine, cursorTo } from "node:readline";

import { yellow } from "colorette";

export const interval = 80;
export const frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"].map(yellow);

export async function spinner<T>(
  title: string,
  promise: (() => Promise<T>) | Promise<T>,
): Promise<Awaited<T>> {
  if (typeof promise === "function") {
    promise = promise();
  }

  const { stdout } = process;

  if (!stdout.isTTY) {
    stdout.write(`${title}\n`);

    return await promise;
  }

  let i = 0;

  const id = setInterval(() => {
    const frame = frames[i]!;
    i = (i + 1) % frames.length;

    try {
      stdout.cork();

      clearLine(stdout, -1);
      cursorTo(stdout, 0);

      stdout.write(`${frame} ${title}`);
    } finally {
      stdout.uncork();
    }
  }, interval);

  try {
    return await promise;
  } finally {
    clearInterval(id);

    try {
      stdout.cork();

      clearLine(stdout, -1);
      cursorTo(stdout, 0);
    } finally {
      stdout.uncork();
    }
  }
}
