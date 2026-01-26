import { getServerSession } from "@midday/auth/server-auth";
import { auth } from "@midday/auth";
import { type Context } from "hono";

export type Session = typeof auth.$Infer.Session;

/**
 * Verify session from Authorization header (Bearer token) or Cookie header
 * This supports both server-to-server (Bearer token) and client-to-server (Cookie) authentication
 */
export async function verifySession(c: Context) {
  const session = await getServerSession(c.req.raw.headers);

  if (!session) {
    return null;
  }

  return {
    session: session.session,
    user: session.user,
  };
}
