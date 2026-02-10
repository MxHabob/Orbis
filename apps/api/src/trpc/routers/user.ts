import { updateUserSchema } from "@api/schemas/users";
import { resend } from "@api/services/resend";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { withRetryOnPrimary } from "@api/utils/db-retry";
import { TRPCError } from "@trpc/server";
import { z } from "zod";
import {
  createUsers,
  deleteUser,
  getStaffUsers,
  getUserById,
  getUserInvites,
  updateUser,
  linkParentToStudents,
} from "@midday/db/queries";
import { generateFileKey } from "@midday/encryption";

export const userRouter = createTRPCRouter({
  getStaff: protectedProcedure.query(async ({ ctx: { db } }) => {
    return getStaffUsers(db);
  }),
  get: protectedProcedure.query(async ({ ctx: { db } }) => {
    return getStaffUsers(db);
  }),
  create: protectedProcedure
    .input(
      z.array(
        z.object({
          name: z.string().min(2),
          email: z.string().email(),
          role: z.enum(["admin", "parent", "teacher", "staff", "student"]),
          // Optional: link students when creating a parent
          studentIds: z.array(z.string().uuid()).optional(),
          relationship: z.enum(["parent", "mother", "father", "guardian"]).optional(),
        }),
      ),
    )
    .mutation(async ({ ctx: { db, session }, input }) => {
      const currentUser = await getUserById(db, session.user.id);

      if (currentUser?.role !== "admin") {
        throw new TRPCError({ code: "FORBIDDEN" });
      }

      const createdUsers = await createUsers(
        db,
        input.map((invite) => ({
          name: invite.name,
          email: invite.email,
          role: invite.role,
        })),
      );

      // Auto-link parents to students if studentIds are provided
      for (const invite of input) {
        if (invite.role === "parent" && invite.studentIds?.length) {
          const createdParent = createdUsers.find((u) => u.email === invite.email);
          if (createdParent) {
            await linkParentToStudents(
              db,
              createdParent.id,
              invite.studentIds.map((sid) => ({
                studentId: sid,
                relationship: invite.relationship ?? "parent",
              })),
            );
          }
        }
      }

      return createdUsers;
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
