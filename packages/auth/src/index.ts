import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";
import { db } from "@orbis/db/client";
import { 
  users, 
  sessions, 
  accounts, 
  verifications 
} from "@orbis/db/schema";

function getApiUrl(): string {
  if (process.env.BETTER_AUTH_URL) {
    return process.env.BETTER_AUTH_URL;
  }
  if (process.env.API_URL) {
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
    origins.push(...process.env.ALLOWED_API_ORIGINS.split(","));
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

const betterAuthSecret = process.env.BETTER_AUTH_SECRET ?? "orbis-dev-secret-change-me";

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
      enabled: true,
      domain: process.env.NODE_ENV === "production" ? ".mulverse.com" : undefined,
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
        domain: process.env.NODE_ENV === "production" ? ".mulverse.com" : undefined,
      },
    };
  },
});

export { hashPassword, verifyPassword } from "better-auth/crypto";
