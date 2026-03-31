
import { CertificateEditor } from "@/components/certificates/certificate-editor";

export const metadata = {
  title: "Create Template | Mommy",
};

export default function CreateTemplatePage() {
  return <CertificateEditor mode="create" />;
}
