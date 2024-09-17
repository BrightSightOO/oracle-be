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
