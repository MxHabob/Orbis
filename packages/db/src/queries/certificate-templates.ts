import { db } from "@db/client";
import { certificateTemplates } from "@db/schema";
import { eq, asc } from "drizzle-orm";

export type CertificateTemplateInsert = typeof certificateTemplates.$inferInsert;
export type CertificateTemplate = typeof certificateTemplates.$inferSelect;

export async function listCertificateTemplates() {
  return db
    .select()
    .from(certificateTemplates)
    .orderBy(asc(certificateTemplates.name));
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
    name: data.name ?? "الافتراضي",
    isDefault: data.isDefault ?? false,
    
    // Layout
    orientation: data.orientation ?? "portrait",
    size: data.size ?? "a4",
    primaryColor: data.primaryColor ?? "#000000",
    
    // Content
    headerContent: data.headerContent ?? null,
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
