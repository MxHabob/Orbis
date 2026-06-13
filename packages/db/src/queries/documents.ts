import { db } from "@db/client";
import { documents } from "@db/schema";
import { and, desc, eq, ilike, sql } from "drizzle-orm";

export type CreateDocumentParams = typeof documents.$inferInsert;

export const getD = async (
  userId: string,
  filter: {
    q?: string | null;
    offset?: number;
    pageSize?: number;
  },
) => {
  const { q, offset = 0, pageSize = 20 } = filter;

  const where = []; // Removed userId filter so all school users share files

  if (q) {
    where.push(ilike(documents.name, `%${q}%`));
  }

  const data = await db.query.documents.findMany({
    where: where.length > 0 ? and(...where) : undefined,
    orderBy: [desc(documents.createdAt)],
    limit: pageSize,
    offset: offset,
  });

  return data;
};

export const createDocument = async (data: CreateDocumentParams) => {
  const [result] = await db.insert(documents).values(data).returning();
  return result;
};

export const deleteDocument = async (id: string, userId: string) => {
  const [result] = await db
    .delete(documents)
    .where(eq(documents.id, id))
    .returning();
  return result;
};

export const updateDocument = async (
  id: string,
  userId: string,
  data: Partial<CreateDocumentParams>,
) => {
  const [result] = await db
    .update(documents)
    .set({ ...data, updatedAt: new Date() })
    .where(eq(documents.id, id))
    .returning();
  return result;
};
