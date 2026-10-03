<!--
  @component CreateDatabaseDialog
  Interactive modal wizard for creating a brand new SQLite database with schema migrations.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
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
    errorMessage = l("اسم ملف قاعدة البيانات مطلوب", "Database filename is required");
    return;
  }

  isSubmitting = true;
  errorMessage = null;

  try {
    await controller.createNewDatabase({
      customDir: customDir.trim() || undefined,
      filename: filename.trim(),
    });
    toast.success(l("تم إنشاء وتهيئة قاعدة البيانات بنجاح", "Database created and initialized successfully"));
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
            {m.database_create_dialog_title?.() || l("إنشاء قاعدة بيانات جديدة", "Create New Database")}
          </DialogTitle>
          <DialogDescription class="text-xs text-muted-foreground mt-0.5">
            {m.database_create_dialog_desc?.() || l("تهيئة ملف قاعدة بيانات SQLite وتطبيق مخططات Orbis تلقائياً", "Initialize SQLite database and run schema migrations")}
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
      <FormField id="customDir" label={m.database_create_folder_label?.() || l("مجلد التخزين (اختياري)", "Storage Folder (Optional)")}>
        {#snippet children()}
          <div class="flex gap-2">
            <Input
              id="customDir"
              type="text"
              bind:value={customDir}
              placeholder={l("المجلد الافتراضي للتطبيق", "Default Application Data Folder")}
              class="h-9 text-xs font-mono bg-background border-input flex-1"
              style="direction: ltr;"
            />
            <Button
              type="button"
              variant="outline"
              size="sm"
              class="h-9 px-3 text-xs gap-1.5 shrink-0 border-border"
              onclick={handleBrowseFolder}
            >
              <FolderOpen size={14} class="text-primary" />
              <span>{m.database_browse_folder?.() || l("تصفح", "Browse")}</span>
            </Button>
          </div>
        {/snippet}
      </FormField>

      <!-- Database Filename -->
      <FormField id="filename" label={m.database_create_filename_label?.() || l("اسم ملف قاعدة البيانات", "Database File Name")} required>
        {#snippet children()}
          <Input
            id="filename"
            type="text"
            bind:value={filename}
            placeholder="orbis.db"
            class="h-9 text-xs font-mono bg-background border-input"
            style="direction: ltr;"
          />
        {/snippet}
      </FormField>

      <div class="p-3 rounded-xl bg-muted/40 border border-border/50 text-[11px] text-muted-foreground space-y-1">
        <div class="flex items-center gap-1.5 font-semibold text-foreground">
          <Sparkles size={12} class="text-primary" />
          <span>{l("العمليات التلقائية عند الإنشاء:", "Automatic operations on creation:")}</span>
        </div>
        <ul class="list-disc list-inside space-y-0.5 text-muted-foreground ps-1">
          <li>{l("تفعيل نمط التزامن الفائق (WAL Mode)", "Enable SQLite WAL Concurrency Mode")}</li>
          <li>{l("تطبيق جميع ترحيلات الجداول والمؤشرات", "Apply all database schema migrations")}</li>
          <li>{l("فحص وتأكيد سلامة الملف فور الإنشاء", "Immediate integrity check & connection switch")}</li>
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
        {l("إلغاء", "Cancel")}
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
          <ArrowPath size={14} class="animate-spin" />
          <span>{l("جاري الإنشاء والتهيئة...", "Creating & Initializing...")}</span>
        {:else}
          <Database size={14} />
          <span>{m.database_create_confirm_btn?.() || l("إنشاء وتهيئة القاعدة", "Create & Initialize")}</span>
        {/if}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
