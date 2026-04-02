import { verifySession } from "@api/utils/auth";
import type { Session, HonoEnv } from "@api/utils/auth";
import { getGeoContext } from "@api/utils/geo";
import type { Database } from "@midday/db/client";
import { db } from "@midday/db/client";
import { TRPCError, initTRPC } from "@trpc/server";
import type { Context } from "hono";
import superjson from "superjson";
import { withPrimaryReadAfterWrite } from "./middleware/primary-read-after-write";
import { withPermission } from "./middleware/permission";

type TRPCContext = {
  session: Session | null;
  db: Database;
  geo: ReturnType<typeof getGeoContext>;
  teamId?: string;
  forcePrimary?: boolean;
  role?: string;
  employeeRoles?: string[];
};

export const createTRPCContext = async (
  _: unknown,
  c: Context<HonoEnv>,
): Promise<TRPCContext> => {
  const authResult = await verifySession(c);
  const session = authResult ? authResult : null;

  // Use the singleton database instance - no need for caching
  const geo = getGeoContext(c.req);

  // Check if client wants to force primary database reads (for replication lag handling)
  const forcePrimary = c.req.header("x-force-primary") === "true";

  return {
    session,
    db,
    geo,
    forcePrimary,
  };
};

const t = initTRPC.context<TRPCContext>().create({
  transformer: superjson,
});

export const createTRPCRouter = t.router;
export const createCallerFactory = t.createCallerFactory;

const withPrimaryDbMiddleware = t.middleware(async (opts) => {
  return withPrimaryReadAfterWrite({
    ctx: opts.ctx,
    type: opts.type,
    next: opts.next,
  });
});

const withPermissionMiddleware = t.middleware(async (opts) => {
  return withPermission({
    ctx: opts.ctx,
    next: opts.next,
  });
});

export const publicProcedure = t.procedure.use(withPrimaryDbMiddleware);

export const protectedProcedure = t.procedure
  .use(withPermissionMiddleware) // NOTE: This is needed to ensure that the teamId is set in the context
  .use(withPrimaryDbMiddleware)
  .use(async (opts) => {
    const { session } = opts.ctx;

    if (!session || !session.user) {
      throw new TRPCError({ code: "UNAUTHORIZED" });
    }

    return opts.next({
      ctx: {
        ...opts.ctx,
        session,
      },
    });
  });

export const adminProcedure = protectedProcedure.use(async ({ ctx, next }) => {
  if (ctx.role !== "admin") {
    throw new TRPCError({
      code: "FORBIDDEN",
      message: "This action is restricted to administrators.",
    });
  }
  return next();
});

export const teacherProcedure = protectedProcedure.use(async ({ ctx, next }) => {
  if (ctx.role !== "admin" && ctx.role !== "teacher") {
    throw new TRPCError({
      code: "FORBIDDEN",
      message: "This action is restricted to teachers or administrators.",
    });
  }
  return next();
});
