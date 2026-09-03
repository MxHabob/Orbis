<script lang="ts">
    import { Button } from "../components/ui/button";
    import { Save, FolderOpen, Trash2 } from "@lucide/svelte";
    import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter, DialogTrigger } from "../components/ui/dialog";
    import { Input } from "../components/ui/input";
    import type { UniversalFilter } from "./types";
    import { onMount } from "svelte";

    let {
        filter = $bindable()
    }: {
        filter: UniversalFilter;
    } = $props();

    interface Template {
        id: string;
        name: string;
        filter: UniversalFilter;
    }

    let templates: Template[] = $state([]);
    let templateName = $state("");
    let isSaveDialogOpen = $state(false);
    let isLoadDialogOpen = $state(false);

    onMount(() => {
        const stored = localStorage.getItem('orbis_filter_templates');
        if (stored) {
            try {
                templates = JSON.parse(stored);
            } catch (e) {
                console.error("Failed to load templates", e);
            }
        }
    });

    function saveTemplates() {
        localStorage.setItem('orbis_filter_templates', JSON.stringify(templates));
    }

    function handleSave() {
        if (!templateName.trim()) return;
        
        templates = [...templates, {
            id: Date.now().toString(),
            name: templateName,
            filter: JSON.parse(JSON.stringify(filter))
        }];
        
        saveTemplates();
        templateName = "";
        isSaveDialogOpen = false;
    }

    function loadTemplate(t: Template) {
        filter = JSON.parse(JSON.stringify(t.filter));
        isLoadDialogOpen = false;
    }

    function removeTemplate(id: string) {
        templates = templates.filter(t => t.id !== id);
        saveTemplates();
    }
</script>

<div class="flex items-center gap-2">
    <!-- Save Dialog -->
    <Dialog bind:open={isSaveDialogOpen}>
        <DialogTrigger>
            {#snippet child({ props })}
                <Button variant="outline" size="sm" class="gap-2 text-xs h-8" {...props}>
                    <Save class="w-3.5 h-3.5" /> Save View
                </Button>
            {/snippet}
        </DialogTrigger>
        <DialogContent>
            <DialogHeader>
                <DialogTitle>Save Filter Template</DialogTitle>
                <DialogDescription>Save the current filter configuration for quick access later.</DialogDescription>
            </DialogHeader>
            <div class="py-4">
                <Input placeholder="Template Name" bind:value={templateName} class="text-sm" />
            </div>
            <DialogFooter>
                <Button onclick={handleSave} size="sm">Save</Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>

    <!-- Load Dialog -->
    {#if templates.length > 0}
        <Dialog bind:open={isLoadDialogOpen}>
            <DialogTrigger>
                {#snippet child({ props })}
                    <Button variant="outline" size="sm" class="gap-2 text-xs h-8" {...props}>
                        <FolderOpen class="w-3.5 h-3.5" /> Load View
                    </Button>
                {/snippet}
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Saved Templates</DialogTitle>
                    <DialogDescription>Select a previously saved filter configuration.</DialogDescription>
                </DialogHeader>
                <div class="flex flex-col gap-2 max-h-[300px] overflow-y-auto py-2">
                    {#each templates as t}
                        <div class="flex items-center justify-between p-2 border rounded-md hover:bg-muted/50 transition-colors">
                            <button class="text-sm font-medium flex-1 text-left cursor-pointer" onclick={() => loadTemplate(t)}>
                                {t.name}
                            </button>
                            <Button variant="ghost" size="icon" class="h-8 w-8 text-destructive" onclick={() => removeTemplate(t.id)}>
                                <Trash2 class="w-3.5 h-3.5" />
                            </Button>
                        </div>
                    {/each}
                </div>
            </DialogContent>
        </Dialog>
    {/if}
</div>
