import { db } from "@db/client";
import { certificateTemplates } from "@db/schema";
import { eq, asc, ilike, count, desc } from "drizzle-orm";

export type CertificateTemplateInsert = typeof certificateTemplates.$inferInsert;
export type CertificateTemplate = typeof certificateTemplates.$inferSelect;

export async function listCertificateTemplates(q?: string) {
  let query = db.select({
    id: certificateTemplates.id,
    name: certificateTemplates.name,
    isDefault: certificateTemplates.isDefault,
    size: certificateTemplates.size,
    orientation: certificateTemplates.orientation,
    updatedAt: certificateTemplates.updatedAt,
  }).from(certificateTemplates);

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
  const id = data.id;
  const payload = {
    name: data.name ?? "Default",
    isDefault: data.isDefault ?? false,
    
    // Layout
    orientation: data.orientation ?? "portrait",
    size: data.size ?? "a4",
    primaryColor: data.primaryColor ?? "#000000",
    backgroundImageUrl: data.backgroundImageUrl ?? null,
    
    // Content
    headerContent: data.headerContent ?? null,
    mainContent: data.mainContent ?? null,
    footerContent: data.footerContent ?? null,
    
    // Config
    tableConfig: data.tableConfig ?? null,
    
    updatedAt: new Date(),
  };

  if (id) {
    const [updated] = await db
      .update(certificateTemplates)
      .set(payload)
      .where(eq(certificateTemplates.id, id))
      .returning();
    return updated ?? null;
  }

  const [created] = await db
    .insert(certificateTemplates)
    .values({
      ...payload,
      createdAt: new Date(),
    })
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
