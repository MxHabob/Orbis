import { updateUserSchema } from "@api/schemas/users";
import { resend } from "@api/services/resend";
import { createTRPCRouter, protectedProcedure, publicProcedure, adminProcedure } from "@api/trpc/init";
import { accounts, users as usersSchema } from "@midday/db/schema";
import { and, eq, isNotNull } from "drizzle-orm";
import { withRetryOnPrimary } from "@api/utils/db-retry";
import { TRPCError } from "@trpc/server";
import { z } from "zod";
import {
  createUsers,
  deleteUser,
  getStaffUsers,
  getUserById,
  getParentUsers,
  updateUser,
  linkParentToStudents,
} from "@midday/db/queries";

export const userRouter = createTRPCRouter({
  getStaff: protectedProcedure.query(async ({ ctx: { db } }) => {
    return getStaffUsers(db);
  }),
  getParents: protectedProcedure.query(async ({ ctx: { db } }) => {
    return getParentUsers(db);
  }),
  get: protectedProcedure.query(async ({ ctx: { db } }) => {
    return getStaffUsers(db);
  }),
  create: adminProcedure
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

      if (input.length > 0 && createdUsers.length === 0) {
        throw new TRPCError({
          code: "CONFLICT",
          message: "User(s) already exist with these email addresses.",
        });
      }

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

  update: adminProcedure
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

  delete: adminProcedure
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

  getAccountStatus: publicProcedure
    .input(z.object({ email: z.string().email() }))
    .query(async ({ ctx: { db }, input }: { ctx: any; input: { email: string } }) => {
      const [user] = await db
        .select()
        .from(usersSchema)
        .where(eq(usersSchema.email, input.email))
        .limit(1);

      if (!user) {
        return { status: "absent" };
      }

      const [account] = await db
        .select()
        .from(accounts)
        .where(
          and(
            eq(accounts.userId, user.id as string),
            isNotNull(accounts.password)
          )
        )
        .limit(1);

      return {
        status: account ? "exists" : ("needs_password" as const),
        userId: user.id as string,
      };
    }),

  updatePassword: protectedProcedure
    .input(z.object({ password: z.string().min(8) }))
    .mutation(async ({ ctx: { db, session }, input }) => {
      const { hashPassword } = await import("@midday/auth");
      const hashedPassword = await hashPassword(input.password);

      // Check if user already has a password account
      const existingAccount = await db.query.accounts.findFirst({
        where: (accounts, { and, eq }) => 
          and(
            eq(accounts.userId, session.user.id),
            eq(accounts.providerId, "email")
          ),
      });

      if (existingAccount) {
        await db
          .update(accounts)
          .set({ password: hashedPassword })
          .where(eq(accounts.id, existingAccount.id));
      } else {
        // Create new email account for password login
        await db.insert(accounts).values({
          userId: session.user.id,
          providerId: "email",
          accountId: session.user.email,
          password: hashedPassword,
        });
      }

      return { success: true };
    }),
});
