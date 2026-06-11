import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { shortLinks } from "@orbis/db/schema";
import { TRPCError } from "@trpc/server";
import { nanoid } from "nanoid";
import { z } from "zod";

export const shortLinksRouter = createTRPCRouter({
  createForDocument: protectedProcedure
    .input(
      z.object({
        filePath: z.string(),
        expireIn: z
          .number()
          .optional()
          .default(60 * 60 * 24 * 30), // 30 days
      }),
    )
    .mutation(async ({ ctx: { session, db }, input }) => {
      const code = nanoid(10);
      const expiresAt = new Date(Date.now() + input.expireIn * 1000);

      // The short link should point to our API redirector
      const url = `${process.env.NEXT_PUBLIC_API_URL}/files/proxy?filePath=vault/${input.filePath}`;

      const [link] = await db
        .insert(shortLinks)
        .values({
          code,
          url,
          userId: session.user.id,
          expiresAt,
        })
        .returning();

      if (!link) {
        throw new TRPCError({
          code: "INTERNAL_SERVER_ERROR",
          message: "Failed to create short link",
        });
      }

      // We return the short URL that will be handled by our API redirector
      const baseUrl =
        process.env.NEXT_PUBLIC_API_URL || "http://localhost:3002";

      return {
        shortUrl: `${baseUrl}/s/${code}`,
      };
    }),
});
