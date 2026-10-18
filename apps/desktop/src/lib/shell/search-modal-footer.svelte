<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { Bolt } from "@orbis/icons";
  import { Kbd } from "@orbis/ui/kbd";
</script>

<div
  class="h-10 px-4 border-t border-border bg-muted/30 flex items-center justify-between text-xs text-muted-foreground select-none shrink-0 rounded-b-xl"
>
  <div class="flex items-center gap-1.5">
    <Bolt class="size-3.5 fill-primary text-primary animate-pulse" />
    <span class="font-bold text-foreground">{m.shared_ui_orbis_search()}</span>
  </div>
  <div class="flex items-center gap-3 font-medium">
    <div class="flex items-center gap-1">
      <Kbd>↑↓</Kbd>
      <span>{m.common_search_footer_navigate()}</span>
    </div>
    <div class="flex items-center gap-1">
      <Kbd>Enter</Kbd>
      <span>{m.common_search_footer_select()}</span>
    </div>
    <div class="flex items-center gap-1">
      <Kbd>Esc</Kbd>
      <span>{m.common_titlebar_close()}</span>
    </div>
  </div>
</div>
