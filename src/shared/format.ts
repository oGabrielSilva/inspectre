export function formatHex(value: number): string {
  return `0x${value.toString(16).toUpperCase().padStart(2, '0')}`;
}

export function formatMhz(value: NonNullable<unknown>): string {
  const mhz = Number(value);
  return mhz >= 1000 ? `${(mhz / 1000).toFixed(2)} GHz` : `${mhz} MHz`;
}

export function formatCacheSize(kb: number): string {
  return kb >= 1024 ? `${(kb / 1024).toFixed(0)} MB` : `${kb} KB`;
}

export function formatBytes(bytes: number, fractionDigits = 1): string {
  if (bytes < 1024) return `${bytes} B`;
  const kib = bytes / 1024;
  if (kib < 1024) return `${kib.toFixed(fractionDigits)} KiB`;
  const mib = kib / 1024;
  if (mib < 1024) return `${mib.toFixed(fractionDigits)} MiB`;
  const gib = mib / 1024;
  if (gib < 1024) return `${gib.toFixed(fractionDigits)} GiB`;
  const tib = gib / 1024;
  return `${tib.toFixed(fractionDigits)} TiB`;
}

export function formatUptime(seconds: number): string {
  if (seconds < 60) return `${seconds}s`;
  const d = Math.floor(seconds / 86400);
  const h = Math.floor((seconds % 86400) / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  if (d > 0) return `${d}d ${h}h ${m}m`;
  if (h > 0) return `${h}h ${m}m`;
  return `${m}m`;
}
