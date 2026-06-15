/**
 * Local-first file URL helpers — no embedded HTTP server.
 */
import { convertFileSrc } from "@tauri-apps/api/core";

export function getUnifiedApiUrl(): string {
  return "";
}

export function getFileProxyUrl(filePath: string): string {
  if (!filePath) return "";

  if (filePath.startsWith("http://") || filePath.startsWith("https://")) {
    return filePath;
  }

  if (filePath.startsWith("asset://") || filePath.startsWith("https://asset.")) {
    return filePath;
  }

  try {
    return convertFileSrc(filePath);
  } catch {
    return filePath;
  }
}

export function resolveDocumentUrl(
  doc: { url?: string | null; key?: string | null; path?: string | null },
): string {
  if (doc.url) return getFileProxyUrl(doc.url);
  if (doc.path) return getFileProxyUrl(doc.path);
  if (doc.key) return getFileProxyUrl(doc.key);
  return "";
}
