<!--
  @component CreateDatabaseDialog
  Interactive modal wizard for creating a brand new SQLite database with schema migrations.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";

import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FormField } from "@orbis/ui";
import { toast } from "@orbis/ui";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@orbis/ui/dialog";
import { ArrowPath, Database, FolderOpen, InfoCircle, Sparkles } from "@orbis/icons";
import type { DatabaseManagerController } from "./database-manager.svelte";

let {
  open = $bindable(false),
  controller,
}: {
  open: boolean;
  controller: DatabaseManagerController;
} = $props();

let customDir = $state("");
let filename = $state("orbis.db");
let isSubmitting = $state(false);
let errorMessage = $state<string | null>(null);

async function handleBrowseFolder() {
  const folder = await controller.pickFolder();
  if (folder) {
    customDir = folder;
  }
}

async function handleCreate() {
  if (!filename.trim()) {
    errorMessage = m.ui_copy_database_filename_is_required_f6924780();
    return;
  }

  isSubmitting = true;
  errorMessage = null;

  try {
    await controller.createNewDatabase({
      customDir: customDir.trim() || undefined,
      filename: filename.trim(),
    });
    toast.success(m.ui_copy_database_created_and_initialized_successfu_6e09e55d());
    open = false;
  } catch (e) {
    errorMessage = e instanceof Error ? e.message : String(e);
    toast.error(errorMessage);
  } finally {
    isSubmitting = false;
  }
}
</script>

<Dialog bind:open>
  <DialogContent class="max-w-md text-start font-sans">
    <DialogHeader>
      <div class="flex items-center gap-3">
        <div class="size-9 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary">
          <Database size={18} />
        </div>
        <div>
          <DialogTitle class="text-base font-bold text-foreground">
            {m.database_create_dialog_title?.() || m.database_action_create()}
          </DialogTitle>
          <DialogDescription class="text-xs text-muted-foreground mt-0.5">
            {m.database_create_dialog_desc?.() || m.ui_copy_initialize_sqlite_database_and_run_schema__58f96b74()}
          </DialogDescription>
        </div>
      </div>
    </DialogHeader>

    <div class="space-y-4 py-2 text-xs">
      {#if errorMessage}
        <div class="p-3 rounded-xl bg-destructive/10 border border-destructive/20 text-destructive text-xs flex items-center gap-2">
          <InfoCircle size={15} class="shrink-0" />
          <span>{errorMessage}</span>
        </div>
      {/if}

      <!-- Folder Selection -->
      <FormField id="customDir" label={m.database_create_folder_label?.() || m.ui_copy_storage_folder_optional_1f6159bb()}>
        {#snippet children()}
          <div class="flex gap-2">
            <Input
              id="customDir"
              type="text"
              bind:value={customDir}
              placeholder={m.ui_copy_default_application_data_folder_cc5cf159()}
              class="h-9 text-xs font-mono bg-background border-input flex-1"
            />
            <Button
              type="button"
              variant="outline"
              size="sm"
              class="h-9 px-3 text-xs gap-1.5 shrink-0 border-border"
              onclick={handleBrowseFolder}
            >
              <FolderOpen size={14} class="text-primary" />
              <span>{m.database_browse_folder?.() || m.ui_copy_browse_f7b87051()}</span>
            </Button>
          </div>
        {/snippet}
      </FormField>

      <!-- Database Filename -->
      <FormField id="filename" label={m.database_create_filename_label?.() || m.database_create_filename_label()} required>
        {#snippet children()}
          <Input
            id="filename"
            type="text"
            bind:value={filename}
            placeholder="orbis.db"
            class="h-9 text-xs font-mono bg-background border-input"
          />
        {/snippet}
      </FormField>

      <div class="p-3 rounded-xl bg-muted/40 border border-border/50 text-[11px] text-muted-foreground space-y-1">
        <div class="flex items-center gap-1.5 font-semibold text-foreground">
          <Sparkles size={12} class="text-primary" />
          <span>{m.ui_copy_automatic_operations_on_creation_e325bdb6()}</span>
        </div>
        <ul class="list-disc list-inside space-y-0.5 text-muted-foreground ps-1">
          <li>{m.ui_copy_enable_sqlite_wal_concurrency_mode_9ba69977()}</li>
          <li>{m.ui_copy_apply_all_database_schema_migrations_0845832a()}</li>
          <li>{m.ui_copy_immediate_integrity_check_connection_switc_bf803157()}</li>
        </ul>
      </div>
    </div>

    <DialogFooter class="gap-2 sm:gap-0">
      <Button
        type="button"
        variant="outline"
        size="sm"
        class="h-9 text-xs"
        onclick={() => open = false}
        disabled={isSubmitting}
      >
        {m.auth_provider_cancel()}
      </Button>

      <Button
        type="button"
        variant="default"
        size="sm"
        class="h-9 text-xs font-bold gap-1.5"
        onclick={handleCreate}
        disabled={isSubmitting}
      >
        {#if isSubmitting}
          <LoadingIndicator   size={14} />
          <span>{m.ui_copy_creating_initializing_cbf4c4bf()}</span>
        {:else}
          <Database size={14} />
          <span>{m.database_create_confirm_btn?.() || m.ui_copy_create_initialize_e16f9b0e()}</span>
        {/if}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
