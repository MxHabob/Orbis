<!--
@component AvatarUpload
A circular avatar image uploader with hover edit states and customizable fallbacks.
-->
<script lang="ts">
import { Avatar, AvatarImage, AvatarFallback } from "@orbis/ui-svelte/avatar";
import { cn } from "@orbis/ui-svelte/utils";
import { Loader2, User } from "lucide-svelte";

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
    "relative flex items-center justify-center rounded-md  group cursor-pointer overflow-hidden bg-muted",
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
    <Avatar class="w-full h-full rounded-md">
      <AvatarImage
        src={avatarUrl ?? undefined}
        alt="Avatar"
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
      <span>Change ▾</span>
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
