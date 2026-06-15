import { useParams } from "react-router-dom";
import { CertificateEditor } from "@/components/certificates/editor/certificate-editor";
import { useTauriQuery } from "@/hooks/use-tauri-query";
import { commands } from "@orbis/desktop-client/commands";
import { queryKeys } from "@/lib/query-keys";
import { Spinner } from "@orbis/ui/spinner";

export default function CertificateEditorPage() {
  const { id } = useParams<{ id: string }>();

  const { data: template, isLoading } = useTauriQuery(
    queryKeys.certificates.detail(id!),
    () => commands.certificates.getById(id!),
    { enabled: !!id }
  );

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-background">
        <Spinner />
      </div>
    );
  }

  if (!template) {
    return (
      <div className="flex items-center justify-center min-h-screen bg-background">
        <p className="text-muted-foreground">Template not found</p>
      </div>
    );
  }

  return <CertificateEditor mode="edit" initialData={template} />;
}
