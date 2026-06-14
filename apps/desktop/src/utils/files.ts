/**
 * Orbis Unified File Manager (2026 Enterprise Best Practices)
 * Centralizes all file upload, download, and proxy URL resolution logic.
 * Eliminates reliance on Next.js config rewrites.
 */

export function getUnifiedApiUrl(): string {
  if (typeof window !== "undefined") {
    const hostname = window.location.hostname;
    const protocol = window.location.protocol;
    const isLocal =
      hostname === "localhost" ||
      hostname === "127.0.0.1" ||
      hostname.startsWith("192.168.") ||
      hostname.startsWith("10.") ||
      /^172\.(1[6-9]|2\d|3[01])\./.test(hostname) ||
      hostname.endsWith(".local");

    if (isLocal) {
      return `${protocol}//${hostname}:3002`;
    }
  } else {
    // Server-side context (e.g. Next.js SSR inside Docker container)
    if (process.env.API_URL) {
      return process.env.API_URL;
    }
  }

  if (process.env.NEXT_PUBLIC_API_URL) {
    return process.env.NEXT_PUBLIC_API_URL;
  }

  return "http://localhost:3002";
}

/**
 * Resolves the absolute proxy URL for displaying/viewing a file.
 * Directly communicates with the API server.
 */
export function getFileProxyUrl(filePath: string): string {
  if (!filePath) return "";

  // If it's already an absolute URL and includes /files/proxy, return it
  if (filePath.startsWith("http://") || filePath.startsWith("https://")) {
    if (filePath.includes("/files/proxy")) {
      return filePath;
    }
    // If it's a direct S3 URL, rewrite it to go through our proxy
    try {
      const url = new URL(filePath);
      if (url.pathname.includes("/vault/") || url.pathname.includes("/avatars/")) {
        const cleanPath = url.pathname.replace(/^\/+/, "");
        const baseUrl = getUnifiedApiUrl();
        return `${baseUrl}/files/proxy?filePath=${encodeURIComponent(cleanPath)}`;
      }
    } catch {}
    return filePath;
  }

  // Ensure filePath starts with bucket name (vault/ or avatars/)
  let cleanPath = filePath.startsWith("/") ? filePath.slice(1) : filePath;
  if (!cleanPath.startsWith("vault/") && !cleanPath.startsWith("avatars/")) {
    cleanPath = `vault/${cleanPath}`;
  }

  const baseUrl = getUnifiedApiUrl();
  return `${baseUrl}/files/proxy?filePath=${encodeURIComponent(cleanPath)}`;
}

/**
 * Unified file upload handler.
 * Uploads a file to a specific bucket with an optional path.
 */
export async function uploadFileUnified(params: {
  file: File;
  bucket?: string;
  path?: string;
}): Promise<{ path: string; url: string }> {
  const { file, bucket = "vault", path } = params;

  const formData = new FormData();
  formData.append("file", file);
  formData.append("bucket", bucket);
  if (path) {
    formData.append("path", path);
  }

  const apiUrl = getUnifiedApiUrl();
  const response = await fetch(`${apiUrl}/api/upload`, {
    method: "POST",
    body: formData,
    credentials: "include",
  });

  if (!response.ok) {
    throw new Error(`Upload failed with status ${response.status}`);
  }

  return response.json();
}

/**
 * Unified download helper to trigger browser download for a file.
 */
export async function triggerFileDownload(filePath: string, filename?: string) {
  const url = getFileProxyUrl(filePath);

  try {
    const response = await fetch(url, { credentials: "include" });
    if (!response.ok) throw new Error("Download request failed");

    const blob = await response.blob();
    const blobUrl = window.URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = blobUrl;
    link.download = filename || filePath.split("/").pop() || "download";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    window.URL.revokeObjectURL(blobUrl);
  } catch (error) {
    console.error("Failed to download file:", error);
    // Fallback: open in a new tab if AJAX fetch fails
    window.open(url, "_blank");
  }
}
