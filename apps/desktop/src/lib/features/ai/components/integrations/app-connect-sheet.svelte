<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type { OrbisApp } from "@orbis/app-store";
import { l } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import AppConnectForm from "./app-connect-form.svelte";
import { Sheet, SheetContent, SheetHeader, SheetTitle } from "@orbis/ui/sheet";

let {
  open = $bindable(false),
  app,
  initialValues = {},
  submitting = false,
  error = null,
  onsubmit,
}: {
  open: boolean;
  app: OrbisApp;
  initialValues?: Record<string, unknown>;
  submitting?: boolean;
  error?: string | null;
  onsubmit: (values: Record<string, unknown>) => void;
} = $props();

const appName = $derived(l(app.name_ar, app.name) ?? app.name);
const formId = $derived(`app-connect-form-${app.id}`);
</script>

<Sheet bind:open={open}>
  <SheetContent
    side="end"
    contentClass="data-[side=end]:w-[min(32rem,calc(100vw-1rem))] data-[side=end]:sm:max-w-[min(32rem,calc(100vw-1rem))]"
    class="flex h-full flex-col overflow-hidden p-0"
  >
    <SheetHeader class="shrink-0 border-b border-border/50 px-6 py-5 text-start">
      <SheetTitle class="text-base font-semibold">
        {m.ui_copy_configuration_9ecff7d2()} — {appName}
      </SheetTitle>
      <p class="text-xs text-muted-foreground">
        {m.ui_integration_connect_for_automation({ name: appName })}
      </p>
    </SheetHeader>
    <div class="min-h-0 flex-1 overflow-y-auto overscroll-contain p-6 scrollbar-hide">
      {#if error}
        <p class="mb-4 text-xs text-destructive" role="alert">{error}</p>
      {/if}
      {#key `${app.id}-${open}`}
        <AppConnectForm
          appId={app.id}
          {formId}
          settings={app.settings}
          {initialValues}
          authenticationRequired={error?.toLowerCase().includes("authentication") ?? false}
          {onsubmit}
        />
      {/key}
    </div>
    <footer class="shrink-0 border-t border-border/50 bg-background px-6 py-4">
      <Button
        type="submit"
        form={formId}
        class="w-full"
        size="sm"
        disabled={submitting}
      >
        {submitting ? m.setup_connection_connecting() : m.ui_copy_save_connect_3d45edcd()}
      </Button>
    </footer>
  </SheetContent>
</Sheet>
