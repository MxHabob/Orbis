import { auth } from "@orbis/auth";
import { type Context } from "hono";

export type Session = typeof auth.$Infer.Session;

export type HonoEnv = {
  Variables: {
    user: typeof auth.$Infer.Session.user | null;
    session: typeof auth.$Infer.Session.session | null;
  };
};

/**
 * Verify session from Hono context
 * The session is already populated by the middleware in index.ts
 */
export async function verifySession(c: Context<HonoEnv>) {
  const session = c.get("session");
  const user = c.get("user");

  if (!session || !user) {
    return null;
  }

  return {
    session,
    user,
  };
}
