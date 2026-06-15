import { Suspense } from "react";
import { useQuery } from "@tanstack/react-query";
import { ScrollableContent } from "@/components/scrollable-content";
import { VaultHeader } from "@/components/vault/vault-header";
import { VaultSkeleton } from "@/components/vault/vault-skeleton";
import { VaultView } from "@/components/vault/vault-view";
import { getInitialTableSettings } from "@/utils/columns";
import { Spinner } from "@orbis/ui/spinner";

export default function VaultPage() {
  const { data: initialSettings, isLoading } = useQuery({
    queryKey: ["vault-table-settings"],
    queryFn: () => getInitialTableSettings("vault"),
  });

  if (isLoading || !initialSettings) {
    return (
      <div className="flex items-center justify-center h-[calc(100vh-200px)]">
        <Spinner />
      </div>
    );
  }

  return (
    <ScrollableContent>
      <VaultHeader />
      <Suspense fallback={<VaultSkeleton />}>
        <VaultView initialSettings={initialSettings} />
      </Suspense>
    </ScrollableContent>
  );
}
