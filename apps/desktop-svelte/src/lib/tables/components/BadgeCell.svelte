<script lang="ts">
  import { Badge } from "@orbis/ui-svelte/badge";

  interface Props {
    row: Record<string, any>;
    column: { id: string; meta?: Record<string, any> };
    api?: any;
    onaction?: (ev: any) => void;
  }

  let { row, column }: Props = $props();

  const value = $derived(String(row[column.id] || ""));

  const variant = $derived.by(() => {
    const val = value.toLowerCase();
    if (["active", "present", "paid", "approved", "open", "success"].includes(val)) return "lime";
    if (["inactive", "absent", "overdue", "failed", "closed", "locked", "withdrawn", "error"].includes(val)) return "destructive";
    if (["pending", "late", "partial", "warning"].includes(val)) return "secondary";
    return "outline";
  });
</script>

<div class="flex items-center w-full h-full px-2 text-start">
  <Badge {variant} class="text-[10px] py-0.5 px-2 font-medium capitalize">
    {value}
  </Badge>
</div>
