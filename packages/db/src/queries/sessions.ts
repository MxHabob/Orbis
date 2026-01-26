import type { Database } from "../client";
import { sessions } from "../schema";
import { eq, and, gt } from "drizzle-orm";

/**
 * Get session token by session ID
 * Returns the token if the session exists and is not expired
 */
export async function getSessionTokenById(
  db: Database,
  sessionId: string,
): Promise<string | null> {
  const sessionData = await db
    .select({ token: sessions.token })
    .from(sessions)
    .where(
      and(
        eq(sessions.id, sessionId),
        gt(sessions.expiresAt, new Date())
      )
    )
    .limit(1);

  if (sessionData.length > 0 && sessionData[0]?.token) {
    return sessionData[0].token;
  }

  return null;
}
