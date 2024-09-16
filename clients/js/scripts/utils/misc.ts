import stringWidth from "string-width";

export async function sleep(timeoutMs: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, timeoutMs));
}

export function formatDuration(secs: number): string {
  const d = (secs / 86_400) >>> 0;
  const h = ((secs % 86_400) / 3_600) >>> 0;
  const m = ((secs % 3_600) / 60) >>> 0;
  const s = secs % 60;

  let fmt = "";

  if (d > 0) {
    fmt += `${d}d`;
  }
  if (h > 0) {
    fmt += `${h}h`;
  }
  if (m > 0) {
    fmt += `${m}m`;
  }
  if (s > 0 || fmt === "") {
    fmt += `${s}s`;
  }

  return fmt;
}

export function wrap(text: string, hardWidth: number): string {
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
