import { useParams } from "react-router-dom";
import { DocumentEditor } from "@/components/document-editor/document-editor";
import { useTauriQuery } from "@/hooks/use-tauri-query";
import { commands } from "@orbis/desktop-client/commands";
import { queryKeys } from "@/lib/query-keys";
import { Spinner } from "@orbis/ui/spinner";

export default function DocumentEditorPage() {
  const { id } = useParams<{ id: string }>();

  const { data: template, isLoading } = useTauriQuery(
    queryKeys.documentTemplates.detail(id!),
    () => commands.documentTemplates.getById(id!),
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

  return <DocumentEditor mode="edit" initialData={template} />;
}
