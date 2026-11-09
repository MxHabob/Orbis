<script lang="ts">
import * as m from "$lib/paraglide/messages";
import type {
  DatabaseManagementSection,
  DatabaseNavigationItem,
} from "./database-management-types";

interface Props {
  activeSection: DatabaseManagementSection;
  items: DatabaseNavigationItem[];
  onSelect: (section: DatabaseManagementSection) => void;
}

let { activeSection, items, onSelect }: Props = $props();
</script>

<nav
  aria-label={m.database_management_title()}
  class="flex shrink-0 gap-5 overflow-x-auto scrollbar-hide border-b border-border text-start"
>
  {#each items as item (item.id)}
    <button
      type="button"
      aria-current={activeSection === item.id ? "page" : undefined}
      class="relative min-h-10 shrink-0 border-b-2 px-0.5 text-xs font-medium transition-colors {activeSection ===
      item.id
        ? 'border-primary text-foreground'
        : 'border-transparent text-muted-foreground hover:text-foreground'}"
      onclick={() => onSelect(item.id)}
    >
      {item.title}
    </button>
  {/each}
</nav>
