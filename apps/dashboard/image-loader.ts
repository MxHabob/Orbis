interface ImageLoaderParams {
  src: string;
  width: number;
  quality?: number;
}

const CDN_URL = "https://orbis.ai";

export default function imageLoader({
  src,
  width,
  quality = 80,
}: ImageLoaderParams): string {
  // Handle authenticated API URLs (preserve query parameters like fk token)
  if (src.includes("/files/proxy")) {
    // Parse URL to preserve query parameters
    try {
      const url = new URL(src);

      const isLocal =
        url.hostname === "localhost" ||
        url.hostname === "127.0.0.1" ||
        url.hostname.startsWith("192.168.") ||
        url.hostname.startsWith("10.") ||
        /^172\.(1[6-9]|2\d|3[01])\./.test(url.hostname) ||
        url.hostname.endsWith(".local");

      if (isLocal) {
        return src; // Return URL as-is for local network
      }

      const params = url.searchParams.toString();
      const baseUrl = url.origin + url.pathname;
      return `${CDN_URL}/cdn-cgi/image/width=${width},quality=${quality}/${baseUrl}${params ? `?${params}` : ""}`;
    } catch {
      // Fallback if URL parsing fails - check if it's local/LAN
      const isLocalSrc =
        src.includes("localhost") ||
        src.includes("127.0.0.1") ||
        src.includes("192.168.") ||
        src.includes("10.") ||
        /172\.(1[6-9]|2\d|3[01])\./.test(src) ||
        src.includes(".local");

      if (isLocalSrc) {
        return src; // Return URL as-is for local network
      }
      return `${CDN_URL}/cdn-cgi/image/width=${width},quality=${quality}/${src}`;
    }
  }

  // Existing logic for other URLs
  if (src.startsWith("/_next")) {
    return `${CDN_URL}/cdn-cgi/image/width=${width},quality=${quality}/https://app.orbis.ai${src}`;
  }
  return `${CDN_URL}/cdn-cgi/image/width=${width},quality=${quality}/${src}`;
}
