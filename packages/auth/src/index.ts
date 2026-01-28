import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";
import { db } from "@midday/db/client";
import { 
  users, 
  sessions, 
  accounts, 
  verifications 
} from "@midday/db/schema";
import { sessionCache, userCache } from "@midday/cache";

// Get API URL from environment or default to localhost
function getApiUrl(): string {
  if (process.env.BETTER_AUTH_URL) {
    return process.env.BETTER_AUTH_URL;
  }
  if (process.env.API_URL) {
    return process.env.API_URL;
  }
  return process.env.NODE_ENV === "production"
    ? "https://api.midday.ai"
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
  databaseHooks: {
    session: {
      create: {
        after: async (session) => {
          await sessionCache.setByToken(session.token, session as any);
          await sessionCache.setById(session.id, session as any);
        },
      },
      update: {
        after: async (session) => {
          if (session) {
             await sessionCache.setByToken(session.token, session as any);
             await sessionCache.setById(session.id, session as any);
          }
        },
      },
    },
    user: {
      create: {
        after: async (user) => {
          await userCache.setById(user.id, user as any);
          await userCache.setByEmail(user.email, user as any);
        },
      },
      update: {
        after: async (user) => {
          if (user) {
            await userCache.setById(user.id, user as any);
            await userCache.setByEmail(user.email, user as any);
            await userCache.invalidate(user.id, user as any); 
          }
        },
      },
    },
  },
  advanced: {
    database: {
      generateId: false,
    },
  },
  emailAndPassword: {
    enabled: true,
  },
  secondaryStorage: {
    get: async (key) => {
      const session = await sessionCache.getByToken(key);
      if (session) {
          return JSON.stringify(session);
      }
      return null;
    },
    set: async (key, value, ttl) => {
        // We rely on database hooks for writing to cache
    },
    delete: async (key) => {
        await sessionCache.invalidate(key);
    }
  }
});
