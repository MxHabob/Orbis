import { CertificateEditor } from "@/components/certificates/editor/certificate-editor";

export const metadata = {
  title: "Create Template | Orbis",
};

export default function CreateTemplatePage() {
  return <CertificateEditor mode="create" />;
}
