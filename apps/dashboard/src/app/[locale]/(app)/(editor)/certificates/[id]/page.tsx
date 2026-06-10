
import { CertificateEditor } from "@/components/certificates/editor/certificate-editor";
import { getCertificateTemplateById } from "@orbis/db/queries/certificate-templates";
import { notFound } from "next/navigation";

export const metadata = {
  title: "Edit Template | Orbis",
};

export default async function EditTemplatePage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = await params;
  const template = await getCertificateTemplateById(id);

  if (!template) {
    notFound();
  }

  return <CertificateEditor mode="edit" initialData={template} />;
}
