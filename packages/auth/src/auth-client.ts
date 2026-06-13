import { magicLinkClient } from "better-auth/client/plugins";
import { createAuthClient } from "better-auth/react";

const getClientApiUrl = (): string => {
  const defaultUrl =
    process.env.NEXT_PUBLIC_API_URL || "https://orbis-api.mulverse.com";
  if (typeof window !== "undefined") {
    try {
      const url = new URL(defaultUrl);
      const defaultHost = url.hostname;
      const isLocalOrIp =
        defaultHost === "localhost" ||
        defaultHost === "127.0.0.1" ||
        /^(\d{1,3}\.){3}\d{1,3}$/.test(defaultHost);

      if (isLocalOrIp) {
        url.hostname = window.location.hostname;
        return url.origin;
      }
    } catch {
      // Fallback
    }
  }
  return defaultUrl;
};

export const authClient = createAuthClient({
  baseURL: getClientApiUrl(),
  plugins: [magicLinkClient()],
});
