import { db } from "@orbis/db/client";
import { accounts, sessions, users, verifications } from "@orbis/db/schema";
import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";

function getApiUrl(): string {
  if (process.env.BETTER_AUTH_URL && process.env.BETTER_AUTH_URL !== "http://" && process.env.BETTER_AUTH_URL !== "https://") {
    return process.env.BETTER_AUTH_URL;
  }
  if (process.env.API_URL && process.env.API_URL !== "http://" && process.env.API_URL !== "https://") {
    return process.env.API_URL;
  }
  return process.env.NODE_ENV === "production"
    ? "https://orbis-api.mulverse.com"
    : "http://localhost:3002";
}

// Get trusted origins (dashboard URLs) from environment
function getTrustedOrigins(): string[] {
  const origins: string[] = [];

  if (process.env.ALLOWED_API_ORIGINS) {
    const parsed = process.env.ALLOWED_API_ORIGINS.split(",")
      .map((o) => o.trim())
      .filter((o) => o && o !== "http://" && o !== "https://");
    origins.push(...parsed);
  }

  if (process.env.DASHBOARD_URL) {
    origins.push(process.env.DASHBOARD_URL);
  }

  if (process.env.NEXT_PUBLIC_URL) {
    origins.push(process.env.NEXT_PUBLIC_URL);
  }

  if (process.env.NEXT_PUBLIC_APP_URL) {
    origins.push(process.env.NEXT_PUBLIC_APP_URL);
  }

  // Default origins for development / local deployment
  if (process.env.NODE_ENV !== "production") {
    const localDefaults = [
      "http://localhost:3001",
      "http://localhost:3002",
      "http://127.0.0.1:3001",
      "http://127.0.0.1:3002",
    ];
    for (const origin of localDefaults) {
      if (!origins.includes(origin)) {
        origins.push(origin);
      }
    }

    // Add server LAN IP if configured (for school network access)
    if (process.env.SERVER_LAN_IP) {
      origins.push(`http://${process.env.SERVER_LAN_IP}:3001`);
      origins.push(`http://${process.env.SERVER_LAN_IP}:3002`);
    }
  }

  return origins.length > 0 ? origins : ["http://localhost:3001"];
}

// Helper to resolve cookie domain dynamically
function getCookieDomain(): string | undefined {
  if (process.env.COOKIE_DOMAIN) {
    return process.env.COOKIE_DOMAIN;
  }

  // Omit domain attribute in development / non-production to default to host-only
  if (process.env.NODE_ENV !== "production") {
    return undefined;
  }

  const appUrl =
    process.env.NEXT_PUBLIC_URL ||
    process.env.NEXT_PUBLIC_APP_URL ||
    process.env.BETTER_AUTH_URL;

  if (appUrl) {
    try {
      const url = new URL(appUrl);
      const hostname = url.hostname;

      // Omit domain for localhost or IP addresses as browsers reject explicit cookie domains for them
      if (
        hostname === "localhost" ||
        hostname === "127.0.0.1" ||
        /^(\d{1,3}\.){3}\d{1,3}$/.test(hostname) ||
        hostname.startsWith("192.168.") ||
        hostname.startsWith("10.") ||
        /^172\.(1[6-9]|2\d|3[01])\./.test(hostname)
      ) {
        return undefined;
      }

      const parts = hostname.split(".");
      if (parts.length >= 2) {
        return `.${parts.slice(-2).join(".")}`;
      }
    } catch {
      // Fallback
    }
  }

  return undefined;
}

const betterAuthSecret =
  process.env.BETTER_AUTH_SECRET ?? "orbis-dev-secret-change-me";

const cookieDomain = getCookieDomain();

export const auth = betterAuth({
  secret: betterAuthSecret,
  baseURL: getApiUrl(),
  trustedOrigins: getTrustedOrigins(),
  database: drizzleAdapter(db, {
    provider: "pg",
    schema: {
      user: users,
      session: sessions,
      account: accounts,
      verification: verifications,
    },
  }),
  advanced: {
    crossSubDomainCookies: {
      enabled: cookieDomain !== undefined,
      domain: cookieDomain,
    },
    database: {
      generateId: false,
    },
    trustProxy: true,
  },
  emailAndPassword: {
    enabled: true,
  },
  magicLink: {
    enabled: true,
  },
  accountLinking: {
    enabled: true,
    trustedProviders: ["google", "apple", "github"],
  },
  onSessionConfig: () => {
    return {
      cookieOptions: {
        httpOnly: true,
        secure: process.env.NODE_ENV === "production",
        sameSite: "lax",
        path: "/",
        domain: cookieDomain,
      },
    };
  },
});

export { hashPassword, verifyPassword } from "better-auth/crypto";
