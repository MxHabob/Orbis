<!--
  Error Recovery Screen — shown when startup fails.
  Clean UI built using @orbis/ui Card, Logo, Button, and standard design tokens.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Card } from "@orbis/ui/card";
import { Button } from "@orbis/ui/button";
import { Logo } from "@orbis/ui/logo";
import { AlertTriangle, RefreshCw, Settings } from "@lucide/svelte";

function retry() {
  goto(localizeHref("/"), { replaceState: true });
}

function reconfigure() {
  goto(localizeHref("/setup/connection"), { replaceState: true });
}
</script>

<svelte:head>
  <title>{m.setup_error_page_title()}</title>
</svelte:head>

<div class="w-full max-w-lg">
  <Card class="bg-card border border-border shadow-2xl rounded-2xl overflow-hidden flex flex-col">
    <!-- Brand Header -->
    <div class="bg-primary/10 border-b border-border/60 p-6 flex items-center gap-4">
      <div class="p-2.5 rounded-xl bg-primary text-primary-foreground">
        <Logo class="size-6 text-primary-foreground" dotClass="fill-white" />
      </div>
      <div>
        <h2 class="text-lg font-bold tracking-tight text-foreground">
          {m.setup_error_heading()}
        </h2>
        <p class="text-xs text-muted-foreground">
          {m.setup_error_subheading()}
        </p>
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="p-6 sm:p-8 flex flex-col gap-6">
      <div class="flex items-start gap-3.5 p-4 rounded-xl bg-amber-500/10 border border-amber-500/20 text-amber-700 dark:text-amber-400">
        <AlertTriangle class="w-5 h-5 shrink-0 mt-0.5" />
        <div class="text-xs leading-relaxed">
          <div class="font-semibold mb-0.5">
            {m.setup_error_card_title()}
          </div>
          <div class="text-muted-foreground">
            {m.setup_error_card_desc()}
          </div>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex flex-col sm:flex-row gap-3 pt-2">
        <Button
          type="button"
          variant="default"
          class="flex-1 inline-flex items-center justify-center gap-2 h-10 text-xs font-semibold cursor-pointer"
          onclick={retry}
        >
          <RefreshCw class="w-3.5 h-3.5" />
          <span>{m.setup_error_retry_btn()}</span>
        </Button>

        <Button
          type="button"
          variant="outline"
          class="flex-1 inline-flex items-center justify-center gap-2 h-10 text-xs font-semibold cursor-pointer border-border"
          onclick={reconfigure}
        >
          <Settings class="w-3.5 h-3.5" />
          <span>{m.setup_error_reconfigure_btn()}</span>
        </Button>
      </div>
    </div>
  </Card>
</div>


