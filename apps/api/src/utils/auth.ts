import { auth } from "@midday/auth";
import { type Context } from "hono";

export type Session = typeof auth.$Infer.Session;

export async function verifySession(c: Context) {
  const session = await auth.api.getSession({
    headers: c.req.raw.headers,
  });

  if (!session) return null;

  return {
    session: session.session,
    user: session.user,
  };
}
