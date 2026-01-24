import { betterAuth } from "better-auth";
import { randomUUID } from "node:crypto";

import { drizzleAdapter } from "better-auth/adapters/drizzle";
import { db } from "@midday/db/client";
import { 
  users, 
  sessions, 
  accounts, 
  verifications 
} from "@midday/db/schema";

export const auth = betterAuth({
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
    database: {
      generateId: false,
    },
  },
  emailAndPassword: {
    enabled: true,
  },
});

