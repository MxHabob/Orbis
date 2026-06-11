import { db } from "@db/client";
import { certificateTemplates } from "@db/schema";
import { asc, count, desc, eq, ilike } from "drizzle-orm";

export type CertificateTemplateInsert =
  typeof certificateTemplates.$inferInsert;
export type CertificateTemplate = typeof certificateTemplates.$inferSelect;

export async function listCertificateTemplates(q?: string) {
  let query = db
    .select({
      id: certificateTemplates.id,
      name: certificateTemplates.name,
      category: certificateTemplates.category,
      isDefault: certificateTemplates.isDefault,
      isArchived: certificateTemplates.isArchived,
      thumbnailUrl: certificateTemplates.thumbnailUrl,
      updatedAt: certificateTemplates.updatedAt,
    })
    .from(certificateTemplates);

  if (q) {
    query = query.where(ilike(certificateTemplates.name, `%${q}%`)) as any;
  }

  return query.orderBy(desc(certificateTemplates.updatedAt));
}

export async function getCertificateTemplateById(id: string) {
  const [row] = await db
    .select()
    .from(certificateTemplates)
    .where(eq(certificateTemplates.id, id))
    .limit(1);
  return row ?? null;
}

export async function getDefaultCertificateTemplate() {
  const [row] = await db
    .select()
    .from(certificateTemplates)
    .where(eq(certificateTemplates.isDefault, true))
    .limit(1);
  return row ?? null;
}

export async function upsertCertificateTemplate(
  data: Partial<CertificateTemplateInsert> & { id?: string },
) {
  const { id, ...rest } = data;

  if (id) {
    const updatePayload: any = { updatedAt: new Date() };

    // Support flexible design tree updates (Figma Style)
    if (rest.name !== undefined) updatePayload.name = rest.name;
    if (rest.description !== undefined)
      updatePayload.description = rest.description;
    if (rest.isDefault !== undefined) updatePayload.isDefault = rest.isDefault;
    if (rest.isArchived !== undefined)
      updatePayload.isArchived = rest.isArchived;
    if (rest.category !== undefined) updatePayload.category = rest.category;
    if (rest.version !== undefined) updatePayload.version = rest.version;
    if (rest.tags !== undefined) updatePayload.tags = rest.tags;
    if (rest.thumbnailUrl !== undefined)
      updatePayload.thumbnailUrl = rest.thumbnailUrl;
    if (rest.content !== undefined) updatePayload.content = rest.content;

    const [updated] = await db
      .update(certificateTemplates)
      .set(updatePayload)
      .where(eq(certificateTemplates.id, id))
      .returning();
    return updated ?? null;
  }

  const payload: CertificateTemplateInsert = {
    name: data.name ?? "Untitled Design",
    description: data.description ?? null,
    isDefault: data.isDefault ?? false,
    isArchived: data.isArchived ?? false,
    category: data.category ?? "design",
    version: data.version ?? 1,
    tags: data.tags ?? [],
    content: data.content ?? { elements: [], canvasConfig: {} },
    thumbnailUrl: data.thumbnailUrl ?? null,
    createdAt: new Date(),
    updatedAt: new Date(),
  };

  const [created] = await db
    .insert(certificateTemplates)
    .values(payload)
    .returning();
  return created ?? null;
}

export async function createCertificateTemplate(
  data: Omit<CertificateTemplateInsert, "id" | "createdAt" | "updatedAt">,
) {
  const [row] = await db
    .insert(certificateTemplates)
    .values({
      ...data,
      createdAt: new Date(),
      updatedAt: new Date(),
    })
    .returning();
  return row ?? null;
}

export async function setDefaultCertificateTemplate(id: string) {
  await db
    .update(certificateTemplates)
    .set({ isDefault: false, updatedAt: new Date() });
  const [updated] = await db
    .update(certificateTemplates)
    .set({ isDefault: true, updatedAt: new Date() })
    .where(eq(certificateTemplates.id, id))
    .returning();
  return updated ?? null;
}

export async function deleteCertificateTemplate(id: string) {
  const [deleted] = await db
    .delete(certificateTemplates)
    .where(eq(certificateTemplates.id, id))
    .returning();
  return deleted ?? null;
}

export async function getCertificateTemplatesSummary() {
  const [counts] = await db
    .select({
      total: count(),
    })
    .from(certificateTemplates);

  const [defaults] = await db
    .select({
      count: count(),
    })
    .from(certificateTemplates)
    .where(eq(certificateTemplates.isDefault, true));

  const [lastUpdate] = await db
    .select({
      date: certificateTemplates.updatedAt,
    })
    .from(certificateTemplates)
    .orderBy(desc(certificateTemplates.updatedAt))
    .limit(1);

  return {
    total: counts?.total ?? 0,
    defaultCount: defaults?.count ?? 0,
    lastUpdated: lastUpdate?.date ?? null,
  };
}
