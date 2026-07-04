<!--
@component LogoUpload
A styled logo uploader with loading indicator, image preview, and hover states.
-->
<script lang="ts">
import { Avatar, AvatarImage, AvatarFallback } from "@orbis/ui-svelte/avatar";
import { Loader2, Upload } from "lucide-svelte";

let {
  logoUrl = $bindable(null),
  size = 64,
  isLoading = false,
  onupload,
}: {
  logoUrl?: string | null;
  size?: number;
  isLoading?: boolean;
  onupload?: (url: string) => void;
} = $props();

let fileInputRef: HTMLInputElement | undefined = $state();

function triggerFileInput() {
  if (isLoading) return;
  fileInputRef?.click();
}

function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;

  // For Tauri/Offline environments, we read as DataURL to trigger preview and onupload
  const reader = new FileReader();
  reader.onload = () => {
    const resultUrl = reader.result as string;
    logoUrl = resultUrl;
    if (onupload) {
      onupload(resultUrl);
    }
  };
  reader.readAsDataURL(file);
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
  {#if isLoading}
    <div class="flex items-center justify-center w-full h-full bg-background/50 backdrop-blur-xs">
      <Loader2 class="animate-spin text-primary shrink-0" size={Math.max(16, size / 3)} />
    </div>
  {:else}
    <Avatar class="w-full h-full rounded-none bg-transparent flex items-center justify-center">
      {#if logoUrl}
        <AvatarImage
          src={logoUrl}
          alt="Logo"
          class="w-full h-full object-contain p-1"
        />
      {:else}
        <AvatarFallback class="bg-transparent flex flex-col items-center justify-center text-muted-foreground transition-colors group-hover:text-primary">
          <Upload size={Math.max(16, size / 3)} class="transition-transform group-hover:-translate-y-0.5" />
        </AvatarFallback>
      {/if}
    </Avatar>

    <!-- Overlay on Hover -->
    {#if logoUrl}
      <div class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity">
        <Upload class="text-white" size={Math.max(16, size / 3)} />
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
