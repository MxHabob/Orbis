import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";
import { db } from "@midday/db/client";
import { 
  users, 
  sessions, 
  accounts, 
  verifications 
} from "@midday/db/schema";

function getApiUrl(): string {
  if (process.env.BETTER_AUTH_URL) {
    return process.env.BETTER_AUTH_URL;
  }
  if (process.env.API_URL) {
    return process.env.API_URL;
  }
  return process.env.NODE_ENV === "production"
    ? "https://mommy-api.mulverse.com"
    : "http://localhost:3002";
}

// Get trusted origins (dashboard URLs) from environment
function getTrustedOrigins(): string[] {
  const origins: string[] = [];
  
  // Add from environment variable if set
  if (process.env.ALLOWED_API_ORIGINS) {
    origins.push(...process.env.ALLOWED_API_ORIGINS.split(","));
  }
  
  // Add dashboard URL if set
  if (process.env.DASHBOARD_URL) {
    origins.push(process.env.DASHBOARD_URL);
  }
  
  // Add NEXT_PUBLIC_APP_URL if set (for dashboard)
  if (process.env.NEXT_PUBLIC_APP_URL) {
    origins.push(process.env.NEXT_PUBLIC_APP_URL);
  }
  
  // Default origins for development
  if (process.env.NODE_ENV !== "production") {
    if (!origins.includes("http://localhost:3001")) {
      origins.push("http://localhost:3001");
    }
  }
  
  return origins.length > 0 ? origins : ["http://localhost:3001"];
}

export const auth = betterAuth({
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
    },
    database: {
      generateId: false,
    },
  },
  emailAndPassword: {
    enabled: true,
  },
});
