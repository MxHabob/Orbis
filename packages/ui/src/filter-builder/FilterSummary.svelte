<script lang="ts">
    import type { UniversalFilter, FieldDefinition } from "./types";

    let {
        filter,
        fields = []
    }: {
        filter: UniversalFilter;
        fields: FieldDefinition[];
    } = $props();

    function getFieldLabel(id: string) {
        return fields.find(f => f.id === id)?.label || id;
    }

    function summarizeGroup(group: UniversalFilter): string {
        const parts: string[] = [];
        
        if (group.conditions && group.conditions.length > 0) {
            for (const c of group.conditions) {
                if (!c.field) continue;
                const fieldLabel = getFieldLabel(c.field);
                parts.push(`${fieldLabel} ${c.operator} ${c.value !== undefined ? c.value : ''}`);
            }
        }

        if (group.groups && group.groups.length > 0) {
            for (const sub of group.groups) {
                const subSummary = summarizeGroup(sub);
                if (subSummary) parts.push(`(${subSummary})`);
            }
        }

        return parts.join(` ${group.logic} `);
    }

    let summary = $derived(summarizeGroup(filter));
</script>

<div class="text-sm text-muted-foreground p-3 bg-muted/50 rounded border">
    {#if summary}
        <strong>Filter:</strong> {summary}
    {:else}
        <em>No active filters.</em>
    {/if}
</div>
