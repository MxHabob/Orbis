import { updateUserSchema } from "@api/schemas/users";
import { resend } from "@api/services/resend";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { withRetryOnPrimary } from "@api/utils/db-retry";
import { TRPCError } from "@trpc/server";
import { z } from "zod";
import {
  deleteUser,
  getStaffUsers,
  getUserById,
  getUserInvites,
  updateUser,
} from "@midday/db/queries";
import { generateFileKey } from "@midday/encryption";

export const userRouter = createTRPCRouter({
  getStaff: protectedProcedure.query(async ({ ctx: { db } }) => {
    return getStaffUsers(db);
  }),
  me: protectedProcedure.query(async ({ ctx: { db, session } }) => {
    const result = await withRetryOnPrimary(db, async (dbInstance) =>
      getUserById(dbInstance, session.user.id),
    );

    if (!result) {
      return undefined;
    }

    return {
      ...result,
      fileKey: null,
    };
  }),

  update: protectedProcedure
    .input(updateUserSchema)
    .mutation(async ({ ctx: { db, session }, input }) => {
      const id = input.id ?? session.user.id;

      if (id !== session.user.id) {
        const currentUser = await getUserById(db, session.user.id);
        if (currentUser?.role !== "admin") {
          throw new TRPCError({ code: "FORBIDDEN" });
        }
      }

      return updateUser(db, {
        ...input,
        id,
      });
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.string().uuid() }).optional())
    .mutation(async ({ ctx: { db, session }, input }) => {
      const id = input?.id ?? session.user.id;

      if (id !== session.user.id) {
        const currentUser = await getUserById(db, session.user.id);
        if (currentUser?.role !== "admin") {
          throw new TRPCError({ code: "FORBIDDEN" });
        }
      }

      const user = await getUserById(db, id);

      if (!user) {
        throw new TRPCError({ code: "NOT_FOUND" });
      }

      const [data] = await Promise.all([
        deleteUser(db, id),
        resend.contacts.remove({
          email: user.email!,
          audienceId: process.env.RESEND_AUDIENCE_ID!,
        }),
      ]);

      return data;
    }),

  invites: protectedProcedure.query(async ({ ctx: { db, session } }) => {
    if (!session.user.email) {
      return [];
    }

    return getUserInvites(db, session.user.email);
  }),
});
