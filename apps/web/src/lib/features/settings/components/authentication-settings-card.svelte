<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Card, CardContent } from "@orbis/ui/card";

let { org: _org }: { org: unknown } = $props();
</script>

<Card>
  <CardContent class="p-6 text-sm text-muted-foreground">
    {m.settings_desktop_policy_read_only()}
  </CardContent>
</Card>
