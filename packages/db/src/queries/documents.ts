import { db } from "@db/client";
import { documents } from "@db/schema";
import { eq, and, desc, sql, ilike } from "drizzle-orm";

export type CreateDocumentParams = typeof documents.$inferInsert;

export const getD = async (userId: string, filter: { 
  q?: string | null, 
  offset?: number,
  pageSize?: number
}) => {
  const { q, offset = 0, pageSize = 20 } = filter;

  const where = [eq(documents.userId, userId)];

  if (q) {
    where.push(ilike(documents.name, `%${q}%`));
  }

  const data = await db.query.documents.findMany({
    where: and(...where),
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
    .where(and(eq(documents.id, id), eq(documents.userId, userId)))
    .returning();
  return result;
};

export const updateDocument = async (id: string, userId: string, data: Partial<CreateDocumentParams>) => {
  const [result] = await db
    .update(documents)
    .set({ ...data, updatedAt: new Date() })
    .where(and(eq(documents.id, id), eq(documents.userId, userId)))
    .returning();
  return result;
};
