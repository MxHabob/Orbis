import type { Session } from "@api/utils/auth";
import { withRetryOnPrimary } from "@api/utils/db-retry";
import type { Database } from "@orbis/db/client";
import { getUserWithRoles } from "@orbis/db/queries/users";
import { TRPCError } from "@trpc/server";

export const withPermission = async <TReturn>(opts: {
  ctx: {
    session?: Session | null;
    db: Database;
  };
  next: (opts: {
    ctx: {
      session?: Session | null;
      db: Database;
      role: string;
      employeeRoles: string[];
    };
  }) => Promise<TReturn>;
}) => {
  const { ctx, next } = opts;

  const userId = ctx.session?.user?.id;

  if (!userId) {
    throw new TRPCError({
      code: "UNAUTHORIZED",
      message: "No permission to access this resource",
    });
  }

  // Get user details and roles using the helper query
  const userData = await withRetryOnPrimary(
    ctx.db,
    async (db) => {
      return getUserWithRoles(db, userId);
    },
    { retryOnNull: true },
  );

  if (!userData) {
    throw new TRPCError({
      code: "NOT_FOUND",
      message: "User not found",
    });
  }

  return next({
    ctx: {
      session: ctx.session,
      db: ctx.db,
      role: userData.role || "user",
      employeeRoles: userData.employeeRoles,
    },
  });
};
