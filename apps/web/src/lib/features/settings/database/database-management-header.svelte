<script lang="ts">
import type { ConnectionMode } from "$lib/api/types";
import * as m from "$lib/paraglide/messages";
import { ArrowPath, Computer, FolderOpen, GlobeAlt, Plus } from "@orbis/icons";
import { Badge } from "@orbis/ui/badge";
import SplitActionMenu from "$lib/shared/components/split-action-menu.svelte";

interface Props {
  connectionMode: ConnectionMode;
  isLocal: boolean;
  isLoading: boolean;
  isOperating: boolean;
  statusLabel: string | null;
  statusTone: "connected" | "warning" | "disconnected" | null;
  onRefresh: () => void;
  onOpenExistingDatabase: () => void;
  onCreateDatabase: () => void;
}

let {
  connectionMode,
  isLocal,
  isLoading,
  isOperating,
  statusLabel,
  statusTone,
  onRefresh,
  onOpenExistingDatabase,
  onCreateDatabase,
}: Props = $props();

let engineLabel = $derived(
  isLocal
    ? m.settings_ui_database_sqlite_embedded()
    : connectionMode === "direct"
      ? m.database_direct_postgres()
      : m.ui_copy_remote_server_efd114c0(),
);

let actions = $derived([
  {
    id: "refresh",
    label: m.common_refresh(),
    icon: ArrowPath,
    disabled: isLoading || isOperating,
    onSelect: onRefresh,
  },
  {
    id: "open-database",
    label: m.database_action_open(),
    icon: FolderOpen,
    disabled: isOperating,
    onSelect: onOpenExistingDatabase,
  },
]);
</script>

<header class="flex shrink-0 flex-col gap-3 border-b border-border px-4 py-3 sm:flex-row sm:items-center sm:justify-between sm:px-6">
  <div class="min-w-0 text-start">
    <h1 class="text-base font-semibold text-foreground">
      {m.database_management_title()}
    </h1>
  </div>

  <div class="flex flex-wrap items-center gap-2">
    <Badge variant="outline" class="gap-1.5 px-2.5 py-1 text-[10px] font-medium">
      {#if isLocal}
        <Computer size={12} class="text-primary" />
      {:else}
        <GlobeAlt size={12} class="text-primary" />
      {/if}
      <span>{engineLabel}</span>
    </Badge>

    {#if statusLabel}
      <Badge variant="outline" class="gap-1.5 px-2.5 py-1 text-[10px] font-medium">
        <span
          class="size-1.5 rounded-full {statusTone === 'connected'
            ? 'bg-emerald-500'
            : statusTone === 'warning'
              ? 'bg-amber-500'
              : 'bg-destructive'}"
        ></span>
        <span>{statusLabel}</span>
      </Badge>
    {/if}

    <SplitActionMenu
      label={m.database_action_create()}
      menuLabel={m.operations_ui_actions()}
      icon={Plus}
      {actions}
      onprimary={onCreateDatabase}
      disabled={isOperating}
    />
  </div>
</header>
