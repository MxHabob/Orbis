import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { z } from "zod";
import { getD, createDocument, deleteDocument, updateDocument } from "@orbis/db/queries/documents";
import { TRPCError } from "@trpc/server";
import { eq, and, sql } from "drizzle-orm";
import { documents } from "@orbis/db/schema";
import { getDownloadUrl } from "@orbis/storage";

export const documentsRouter = createTRPCRouter({
  // ... existing procedures
  get: protectedProcedure
    .input(
      z.object({
        pageSize: z.number().default(20),
        cursor: z.number().optional(),
        q: z.string().optional().nullable(),
      })
    )
    .query(async ({ ctx: { session }, input }) => {
      const offset = input.cursor ?? 0;
      const data = await getD(session.user.id, {
        q: input.q,
        pageSize: input.pageSize,
        offset,
      });

      return {
        data,
        meta: {
          cursor: data.length === input.pageSize ? offset + input.pageSize : null,
        },
      };
    }),

  processDocument: protectedProcedure
    .input(
      z.array(
        z.object({
          filePath: z.array(z.string()).min(1),
          mimetype: z.string(),
          size: z.number(),
        })
      )
    )
    .mutation(async ({ ctx: { session }, input }) => {
      const results = await Promise.all(
        input.map((item) => {
          const name = item.filePath.at(-1);

          if (!name) {
            throw new TRPCError({
              code: "BAD_REQUEST",
              message: "Invalid file path",
            });
          }

          const parts = name.split(".");
          const extension = parts.length > 1 ? parts.pop() : undefined;
          const title = parts.length > 0 && parts[0] !== "" ? parts.join(".") : name;

          return createDocument({
            name,
            title,
            size: item.size,
            mimetype: item.mimetype,
            extension,
            key: item.filePath.join("/"),
            pathTokens: item.filePath,
            userId: session.user.id,
            type: "file",
            metadata: {
              size: item.size,
              mimetype: item.mimetype,
            },
          });
        })
      );

      return results;
    }),

  reprocessDocument: protectedProcedure
    .input(z.object({ id: z.string().uuid() }))
    .mutation(async () => {
      return { success: true };
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.string().uuid() }))
    .mutation(async ({ ctx: { session, db }, input }) => {
      // 1. Find all documents to be deleted (the file/folder itself and all its children)
      // Since we use 'onDelete: cascade' in the DB, we need to collect the S3 keys
      // BEFORE deleting from the database.
      
      const descendants = await db.execute<{ key: string }>(sql`
        WITH RECURSIVE subordinates AS (
          SELECT id, key, parent_folder_id FROM documents 
          WHERE id = ${input.id}::uuid AND user_id = ${session.user.id}::uuid
          UNION
          SELECT e.id, e.key, e.parent_folder_id FROM documents e
          INNER JOIN subordinates s ON s.id = e.parent_folder_id
        )
        SELECT key FROM subordinates WHERE key IS NOT NULL
      `);

      const keysToDelete = descendants.rows.map((r) => r.key);

      // 2. Delete from DB (Recursive cascade will handle children in DB)
      const result = await deleteDocument(input.id, session.user.id);

      if (!result) {
        throw new TRPCError({ code: "NOT_FOUND" });
      }

      // 3. Delete from S3-compatible storage (RustFS) - Bulk deletion for performance
      if (keysToDelete.length > 0) {
        const { deleteFiles } = require("@orbis/storage");
        await deleteFiles({
          bucket: "vault",
          keys: keysToDelete,
        });
      }

      return { success: true };
    }),

  update: protectedProcedure
    .input(
      z.object({
        id: z.string().uuid(),
        name: z.string().optional(),
        isPinned: z.boolean().optional(),
      })
    )
    .mutation(async ({ ctx: { session }, input }) => {
      const { id, ...data } = input;
      return updateDocument(id, session.user.id, data);
    }),

  getSignedUrl: protectedProcedure
    .input(z.object({ id: z.string().uuid() }))
    .query(async ({ ctx: { session, db }, input }) => {
      // 1. Verify ownership
      const [doc] = await db
        .select()
        .from(documents)
        .where(
          and(
            eq(documents.id, input.id),
            eq(documents.userId, session.user.id)
          )
        );

      if (!doc || !doc.key) {
        throw new TRPCError({ code: "NOT_FOUND" });
      }

      const url = `${process.env.NEXT_PUBLIC_API_URL}/files/proxy?filePath=vault/${doc.key}`;

      return { url };
    }),

  getById: protectedProcedure
    .input(
      z.object({
        id: z.string().uuid().optional().nullable(),
        filePath: z.string().optional().nullable(),
      })
    )
    .query(async ({ ctx: { session, db }, input }) => {
      let filter;

      if (input.id) {
        filter = eq(documents.id, input.id);
      } else if (input.filePath) {
        filter = eq(documents.key, input.filePath);
      } else {
        throw new TRPCError({
          code: "BAD_REQUEST",
          message: "Either id or filePath must be provided",
        });
      }

      const [doc] = await db
        .select()
        .from(documents)
        .where(and(filter, eq(documents.userId, session.user.id)));

      if (!doc) throw new TRPCError({ code: "NOT_FOUND" });

      return doc;
    }),

  getRelatedDocuments: protectedProcedure
    .input(z.object({ id: z.string().uuid(), pageSize: z.number().default(12) }))
    .query(async ({ ctx: { session, db }, input }) => {
      return db
        .select()
        .from(documents)
        .where(
          and(
            eq(documents.userId, session.user.id),
            sql`id != ${input.id}::uuid`
          )
        )
        .limit(input.pageSize);
    }),
});
