<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import type { OrbisApp } from "@orbis/app-store";
  import { l } from "$lib/core/i18n";
  import AppConnectForm from "./app-connect-form.svelte";
  import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";

  let {
    open = $bindable(false),
    app,
    submitting = false,
    error = null,
    onsubmit,
  }: {
    open: boolean;
    app: OrbisApp;
    submitting?: boolean;
    error?: string | null;
    onsubmit: (values: Record<string, unknown>) => void;
  } = $props();
</script>

<Sheet bind:open={open}>
  <SheetContent
    side="end"
    contentClass="data-[side=end]:w-[min(32rem,calc(100vw-1rem))] data-[side=end]:sm:max-w-[min(32rem,calc(100vw-1rem))]"
    class="flex h-full flex-col overflow-hidden p-0"
  >
    <SheetHeader class="shrink-0 border-b border-border/50 px-6 py-5 text-start">
      <SheetTitle class="text-base font-semibold">
        {m.ui_copy_configuration_9ecff7d2()} — {l(app.name_ar, app.name)}
      </SheetTitle>
      <p class="text-xs text-muted-foreground">
        {m.ui_integration_connect_for_automation({ name: l(app.name_ar, app.name) })}
      </p>
    </SheetHeader>
    <div class="flex-1 overflow-y-auto p-6">
      {#if error}
        <p class="mb-4 text-xs text-destructive" role="alert">{error}</p>
      {/if}
      <AppConnectForm
        settings={app.settings}
        {submitting}
        {onsubmit}
      />
    </div>
  </SheetContent>
</Sheet>
