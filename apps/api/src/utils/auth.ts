import { auth } from "@midday/auth";
import { type Context } from "hono";

export type Session = typeof auth.$Infer.Session;

export async function verifySession(c: Context) {
  const cookie = c.req.header("cookie");
  console.log("Cookie header:", cookie);
  
  const session = await auth.api.getSession({
    headers: c.req.raw.headers,
  });

  if (!session) return null;

  return {
    session: session.session,
    user: session.user,
  };
}
