import { db } from "@db/client";
import { sessions, users } from "@db/schema";
import { sessionCache, userCache } from "@orbis/cache";
import { and, desc, eq, gte } from "drizzle-orm";

export const getSession = async (token: string) => {
  const cachedSession = await sessionCache.getByToken(token);

  if (cachedSession) {
    const session = cachedSession as any; // Cast to access properties as cache returns unknown/generic
    const cachedUser = await userCache.get(session.userId);
    if (cachedUser) {
      return { session: session, user: cachedUser };
    }
  }

  const result = await db
    .select({
      user: users,
      session: sessions,
    })
    .from(sessions)
    .innerJoin(users, eq(sessions.userId, users.id))
    .where(eq(sessions.token, token))
    .limit(1);

  const [row] = result;

  if (row) {
    const { user, session } = row;

    // Cache the results
    await sessionCache.setByToken(token, session);
    await userCache.setById(user.id, user); // Generic user cache

    return { session, user };
  }

  return null;
};

export const getValidSessionByUserId = async (userId: string) => {
  const result = await db
    .select()
    .from(sessions)
    .where(
      and(eq(sessions.userId, userId), gte(sessions.expiresAt, new Date())),
    )
    .orderBy(desc(sessions.createdAt))
    .limit(1);

  return result[0];
};
