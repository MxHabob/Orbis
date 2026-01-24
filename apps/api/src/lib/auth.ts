import { betterAuth } from "better-auth";
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
    provider: "pg", // أو "sqlite" أو "mysql" حسب قاعدة البيانات، سنفترض pg بناءً على @midday/db/schema
    schema: {
      user: users,
      session: sessions,
      account: accounts,
      verification: verifications,
    },
  }),
  emailAndPassword: {
    enabled: true,
  },
  // يمكنك إضافة social providers هنا لاحقاً
  // socialProviders: { ... }
});
