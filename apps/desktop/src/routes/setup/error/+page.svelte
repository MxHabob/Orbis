<!--
  Error Recovery Screen — shown when startup fails.
  Clean luxury UI built using @orbis/ui, Logo, Button, and standard design tokens.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Logo } from "@orbis/ui/logo";
import { Badge } from "@orbis/ui/badge";
import { AlertTriangle, RefreshCw, Settings, ShieldAlert } from "@lucide/svelte";
import { l } from "$lib/core/i18n";

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

<div class="w-full max-w-xl mx-auto p-1 sm:p-2 animate-in fade-in duration-300">
  <div class="rounded-2xl sm:rounded-3xl border border-border/80 bg-card/95 shadow-2xl backdrop-blur-xl overflow-hidden flex flex-col">
    <!-- Brand Header -->
    <div class="flex items-center justify-between px-6 sm:px-8 py-4 border-b border-border/60 bg-muted/15">
      <div class="flex items-center gap-3">
        <div class="size-8 rounded-xl bg-destructive/10 border border-destructive/20 flex items-center justify-center">
          <Logo class="size-5 text-destructive" dotClass="fill-destructive" />
        </div>
        <div class="text-start">
          <h1 class="text-sm sm:text-base font-bold text-foreground leading-none">
            {m.setup_error_heading()}
          </h1>
          <p class="text-[11px] text-muted-foreground mt-0.5 font-medium">
            {m.setup_error_subheading()}
          </p>
        </div>
      </div>

      <Badge variant="outline" class="text-[10px] px-2.5 py-1 gap-1 text-destructive bg-destructive/5 border-destructive/20">
        <ShieldAlert size={12} />
        <span>{l("تنبيه نظام", "System Notice")}</span>
      </Badge>
    </div>

    <!-- Main Content Area -->
    <div class="p-6 sm:p-8 flex flex-col gap-5 text-start">
      <div class="flex items-start gap-3.5 p-4 rounded-2xl bg-amber-500/10 border border-amber-500/20 text-amber-700 dark:text-amber-400">
        <AlertTriangle class="w-5 h-5 shrink-0 mt-0.5" />
        <div class="text-xs leading-relaxed">
          <div class="font-bold text-foreground mb-1">
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
          class="flex-1 inline-flex items-center justify-center gap-2 h-9 text-xs font-semibold cursor-pointer"
          onclick={retry}
        >
          <RefreshCw class="w-3.5 h-3.5" />
          <span>{m.setup_error_retry_btn()}</span>
        </Button>

        <Button
          type="button"
          variant="outline"
          class="flex-1 inline-flex items-center justify-center gap-2 h-9 text-xs font-semibold cursor-pointer border-border"
          onclick={reconfigure}
        >
          <Settings class="w-3.5 h-3.5" />
          <span>{m.setup_error_reconfigure_btn()}</span>
        </Button>
      </div>
    </div>
  </div>
</div>
