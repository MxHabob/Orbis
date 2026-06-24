<!--
@component AvatarUpload
A circular avatar image uploader with hover edit states and customizable fallbacks.
-->
<script lang="ts">
import { Avatar, AvatarImage, AvatarFallback } from "@orbis/ui-svelte/avatar";
import { cn } from "@orbis/ui-svelte/utils";
import { Loader2, Camera, User } from "lucide-svelte";

let {
  avatarUrl = $bindable(null),
  size = 64,
  isLoading = false,
  fallbackText = "",
  class: className,
  onupload,
}: {
  avatarUrl?: string | null;
  size?: number;
  isLoading?: boolean;
  fallbackText?: string;
  class?: string;
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

  const reader = new FileReader();
  reader.onload = () => {
    const resultUrl = reader.result as string;
    avatarUrl = resultUrl;
    if (onupload) {
      onupload(resultUrl);
    }
  };
  reader.readAsDataURL(file);
}
</script>

<div 
  class={cn(
    "relative flex items-center justify-center rounded-full border border-border hover:border-primary/50 transition-colors group cursor-pointer overflow-hidden bg-muted",
    className
  )}
  style="width: {size}px; height: {size}px;"
  onclick={triggerFileInput}
  onkeydown={(e) => e.key === 'Enter' && triggerFileInput()}
  role="button"
  tabindex="0"
>
  {#if isLoading}
    <div class="flex items-center justify-center w-full h-full bg-background/50 backdrop-blur-xs">
      <Loader2 class="animate-spin text-primary shrink-0" size={Math.max(16, size / 3.5)} />
    </div>
  {:else}
    <Avatar class="w-full h-full bg-transparent">
      <AvatarImage
        src={avatarUrl ?? undefined}
        alt="Avatar"
        class="w-full h-full object-cover"
      />
      <AvatarFallback class="bg-transparent text-muted-foreground font-semibold flex items-center justify-center">
        {#if fallbackText}
          <span style="font-size: {Math.max(11, size / 2.5)}px;">{fallbackText.slice(0, 2).toUpperCase()}</span>
        {:else}
          <User size={Math.max(16, size / 2.5)} />
        {/if}
      </AvatarFallback>
    </Avatar>

    <!-- Overlay on Hover -->
    <div class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 flex flex-col items-center justify-center transition-opacity text-white gap-0.5">
      <Camera size={Math.max(14, size / 4)} />
      {#if size >= 60}
        <span class="text-[9px] font-medium tracking-tight">Edit</span>
      {/if}
    </div>
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
