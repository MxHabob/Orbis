<!--
@component SettingsPage (Placeholder)
AI Center settings — providers, models, autopilot config, prompts.
-->
<script lang="ts">
import { Settings } from "lucide-svelte";
</script>

<div class="h-full overflow-y-auto">
  <div class="px-8 py-6 max-w-5xl">
    <h1 class="text-xl font-semibold text-foreground mb-1">Settings</h1>
    <p class="text-xs text-muted-foreground mb-6">
      Configure AI providers, models, autopilot behavior, and system prompts.
    </p>

    <div class="border border-border/60 rounded-lg p-8">
      <div class="flex flex-col items-center justify-center py-8">
        <div class="w-12 h-12 rounded-full bg-muted flex items-center justify-center mb-4">
          <Settings size={20} class="text-muted-foreground" />
        </div>
        <h3 class="text-sm font-medium text-foreground mb-1">Settings</h3>
        <p class="text-xs text-muted-foreground text-center max-w-xs">
          AI provider configuration will be available here. Start by connecting
          an AI provider in the Integrations page.
        </p>
      </div>
    </div>
  </div>
</div>
