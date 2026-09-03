<script lang="ts">
import { Avatar, AvatarImage, AvatarFallback } from "@orbis/ui/avatar";

interface Props {
  row: Record<string, any>;
  column: { id: string; meta?: Record<string, any> };
  api?: any;
  onaction?: (ev: any) => void;
}

let { row, column }: Props = $props();

const name = $derived(String(row[column.id] || ""));
const src = $derived(row.avatar_url || row.avatar || "");
const initials = $derived.by(() => {
  if (!name) return "";
  const parts = name.trim().split(/\s+/);
  if (parts.length === 1) {
    return parts[0]!.slice(0, 2).toUpperCase();
  }
  const first = parts[0]?.slice(0, 1) || "";
  const last = parts[parts.length - 1]?.slice(0, 1) || "";
  return (first + last).toUpperCase();
});
</script>

<div class="flex items-center gap-2 px-2 text-start w-full h-full">
  <Avatar class="h-6 w-6">
    {#if src}
      <AvatarImage {src} alt={name} />
    {/if}
    <AvatarFallback class="text-[9px] font-bold">{initials}</AvatarFallback>
  </Avatar>
  <span class="text-xs font-medium text-foreground truncate">{name}</span>
</div>
