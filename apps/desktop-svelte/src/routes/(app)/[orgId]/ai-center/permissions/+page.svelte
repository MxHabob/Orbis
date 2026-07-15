<!--
@component PermissionsPage (Placeholder)
AI Center permissions — risk matrix, approval gates, agent access control.
-->
<script lang="ts">
import { Shield } from "lucide-svelte";
</script>

<div class="h-full overflow-y-auto">
  <div class="px-8 py-6 max-w-5xl">
    <h1 class="text-xl font-semibold text-foreground mb-1">Permissions</h1>
    <p class="text-xs text-muted-foreground mb-6">
      Manage AI agent permissions, risk levels, and approval workflows.
    </p>

    <div class="border border-border/60 rounded-lg p-8">
      <div class="flex flex-col items-center justify-center py-8">
        <div class="w-12 h-12 rounded-full bg-muted flex items-center justify-center mb-4">
          <Shield size={20} class="text-muted-foreground" />
        </div>
        <h3 class="text-sm font-medium text-foreground mb-1">Permissions</h3>
        <p class="text-xs text-muted-foreground text-center max-w-xs">
          Configure which AI tools require approval and set risk level thresholds
          for automated actions.
        </p>
      </div>
    </div>
  </div>
</div>
