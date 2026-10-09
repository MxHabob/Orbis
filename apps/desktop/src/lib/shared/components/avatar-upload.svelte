<!--
@component AvatarUpload
A circular avatar image uploader with hover edit states and customizable fallbacks.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { Avatar, AvatarImage, AvatarFallback } from "@orbis/ui/avatar";
import { cn } from "@orbis/ui/utils";
import { AdjustmentsHorizontal, ArrowPath, User } from "@orbis/icons";
import {
  useUploadFile,
  resolveDocumentUrl,
  resolveEntityDocumentUrl,
} from "$lib/features/documents/queries";

let {
  avatarUrl = $bindable(null),
  file = $bindable<File | null>(null),
  size = 64,
  isLoading = false,
  fallbackText = "",
  class: className,
  entityType = "user",
  entityId,
  folderId,
  category = "avatar",
  deferred = false,
  onupload,
  onchange,
}: {
  avatarUrl?: string | null;
  file?: File | null;
  size?: number;
  isLoading?: boolean;
  fallbackText?: string;
  class?: string;
  entityType?: string;
  entityId: string;
  folderId?: string;
  category?: string;
  deferred?: boolean;
  onupload?: (url: string) => void;
  onchange?: (file: File) => void;
} = $props();

import { onMount } from "svelte";

const uploadMutation = useUploadFile();
let fileInputRef: HTMLInputElement | undefined = $state();
let localPreviewUrl: string | null = $state(null);

let resolvedAvatarUrl: string | null = $state(null);

onMount(() => {
  if (avatarUrl && !localPreviewUrl) {
    resolveEntityDocumentUrl(avatarUrl, entityType, entityId)
      .then((url) => {
        resolvedAvatarUrl = url;
      })
      .catch((error: unknown) => {
        console.error("Failed to load avatar:", error);
      });
  }
});

let displayUrl = $derived(localPreviewUrl || resolvedAvatarUrl);

function triggerFileInput() {
  if (isLoading || uploadMutation.isPending) return;
  fileInputRef?.click();
}

function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement;
  const selectedFile = target.files?.[0];
  if (!selectedFile) return;

  file = selectedFile;
  onchange?.(selectedFile);

  // Local preview immediately
  const reader = new FileReader();
  reader.onload = () => {
    localPreviewUrl = reader.result as string;
  };
  reader.readAsDataURL(selectedFile);

  // If deferred, do NOT upload immediately — the parent form handles upload upon save
  if (deferred) {
    return;
  }

  // Direct Background upload for standalone profile views
  uploadMutation.mutate(
    {
      file: selectedFile,
      name: selectedFile.name,
      entity_type: entityType,
      entity_id: entityId,
      folder_id: folderId,
      category: category,
      visibility: "public",
    },
    {
      onSuccess: (data) => {
        const serverUrl = data.url || data.file_path;
        avatarUrl = serverUrl;
        resolveDocumentUrl(serverUrl, data.id)
          .then((url) => {
            resolvedAvatarUrl = url;
            localPreviewUrl = null;
          })
          .catch((error: unknown) => {
            console.error("Failed to load uploaded avatar:", error);
          });
        if (onupload) onupload(serverUrl);
      },
      onError: (err) => {
        console.error("Failed to upload avatar", err);
        localPreviewUrl = null; // Revert on failure
      },
    },
  );
}
</script>

<div 
  class={cn(
    "relative flex items-center justify-center rounded-md group cursor-pointer overflow-hidden bg-muted",
    className
  )}
  style="width: {size}px; height: {size}px;"
  onclick={triggerFileInput}
  onkeydown={(e) => e.key === 'Enter' && triggerFileInput()}
  role="button"
  tabindex="0"
>
  {#if isLoading || uploadMutation.isPending}
    <div class="flex items-center justify-center w-full h-full bg-background/50 backdrop-blur-xs">
      <LoadingIndicator class="text-primary shrink-0"  size={Math.max(16, size / 3.5)} />
    </div>
  {:else}
    <Avatar class="w-full h-full rounded-md">
      <AvatarImage
        src={displayUrl ?? undefined}
        alt={m.shared_ui_avatar()}
        class="w-full h-full object-cover"
      />
      <AvatarFallback class="flex items-center justify-center select-none">
        {#if fallbackText}
          <span style="font-size: {Math.max(16, size / 1.8)}px;">{fallbackText.slice(0, 2).toUpperCase()}</span>
        {:else}
          <User size={Math.max(16, size / 2)} />
        {/if}
      </AvatarFallback>
    </Avatar>

    <!-- Overlay on Hover -->
    <div class="absolute inset-0 bg-black/15 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity text-white font-medium select-none text-xs">
      <span>{m.shared_ui_change()}</span>
    </div>
  {/if}

  <input
    bind:this={fileInputRef}
    type="file"
    accept="image/*"
    onchange={handleFileChange}
    class="hidden"
  />
</div>
