<!--
@component LogoUpload
A styled logo uploader with loading indicator, image preview, and hover states.
-->
<script lang="ts">
import { Avatar, AvatarImage, AvatarFallback } from "@orbis/ui/avatar";
import { AdjustmentsHorizontal, ArrowPath, ArrowUpTray } from "@orbis/icons";
import {
  useUploadDocument,
  resolveDocumentUrl,
} from "$lib/features/documents/queries";

let {
  logoUrl = $bindable(null),
  size = 64,
  isLoading = false,
  entityType = "organization",
  entityId,
  category = "logo",
  onupload,
}: {
  logoUrl?: string | null;
  size?: number;
  isLoading?: boolean;
  entityType?: string;
  entityId: string;
  category?: string;
  onupload?: (url: string) => void;
} = $props();

const uploadMutation = useUploadDocument();
let fileInputRef: HTMLInputElement | undefined = $state();
let localPreviewUrl: string | null = $state(null);

let resolvedLogoUrl: string | null = $state(null);
$effect(() => {
  if (logoUrl && !localPreviewUrl) {
    resolveDocumentUrl(logoUrl).then((url) => {
      resolvedLogoUrl = url;
    });
  }
});

let displayUrl = $derived(localPreviewUrl || resolvedLogoUrl);

function triggerFileInput() {
  if (isLoading || uploadMutation.isPending) return;
  fileInputRef?.click();
}

function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;

  const reader = new FileReader();
  reader.onload = () => {
    localPreviewUrl = reader.result as string;
  };
  reader.readAsDataURL(file);

  uploadMutation.mutate(
    {
      file,
      entity_type: entityType,
      entity_id: entityId,
      category: category,
      visibility: "public",
    },
    {
      onSuccess: (data: any) => {
        const serverUrl = data.url;
        logoUrl = serverUrl;
        if (onupload) onupload(serverUrl);
      },
      onError: (err) => {
        console.error("Failed to upload logo", err);
        localPreviewUrl = null;
      },
    },
  );
}
</script>

<div 
  class="relative flex items-center justify-center border border-dashed border-border rounded-xl hover:border-primary/50 transition-colors group cursor-pointer overflow-hidden bg-muted/30"
  style="width: {size}px; height: {size}px;"
  onclick={triggerFileInput}
  onkeydown={(e) => e.key === 'Enter' && triggerFileInput()}
  role="button"
  tabindex="0"
>
  {#if isLoading || uploadMutation.isPending}
    <div class="flex items-center justify-center w-full h-full bg-background/50 backdrop-blur-xs">
      <ArrowPath class="animate-spin text-primary shrink-0" size={Math.max(16, size / 3)} />
    </div>
  {:else}
    <Avatar class="w-full h-full rounded-none bg-transparent flex items-center justify-center">
      {#if displayUrl}
        <AvatarImage
          src={displayUrl}
          alt="Logo"
          class="w-full h-full object-contain p-1"
        />
      {:else}
        <AvatarFallback class="bg-transparent flex flex-col items-center justify-center text-muted-foreground transition-colors group-hover:text-primary">
          <ArrowUpTray size={Math.max(16, size / 3)} class="transition-transform group-hover:-translate-y-0.5" />
        </AvatarFallback>
      {/if}
    </Avatar>

    <!-- Overlay on Hover -->
    {#if displayUrl}
      <div class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity">
        <ArrowUpTray class="text-white" size={Math.max(16, size / 3)} />
      </div>
    {/if}
  {/if}

  <!-- Hidden Native File Input -->
  <input
    bind:this={fileInputRef}
    type="file"
    accept="image/*"
    onchange={handleFileChange}
    class="hidden"
  />
</div>
