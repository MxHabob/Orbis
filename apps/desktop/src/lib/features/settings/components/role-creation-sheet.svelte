<!--
  @component RoleCreationSheet
  Slide-over Sheet component for creating and editing roles and permissions,
  matching the exact architecture and styling of Orbis Sheets (e.g. PeriodForm, FeeTemplateForm).
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";
import { FormField } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Checkbox } from "@orbis/ui/checkbox";
import { Badge } from "@orbis/ui/badge";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
} from "@orbis/ui/sheet";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@orbis/ui/accordion";
import { useQueryClient } from "@tanstack/svelte-query";
import { toast } from "@orbis/ui";
import type { RoleDto } from "$lib/api/types";
import {
  IAM_DOMAINS,
  type IamResourceDef,
  countRolePermissions,
  translateIamMessage,
} from "$lib/core/iam";

let {
  open = $bindable(false),
  editingRole = null,
  onsave,
}: {
  open: boolean;
  editingRole: RoleDto | null;
  onsave?: (role: RoleDto) => Promise<void> | void;
} = $props();

const queryClient = useQueryClient();
const isEditMode = $derived(Boolean(editingRole));

// Form values
let roleId = $state("");
let roleName = $state("");
let roleNameAr = $state("");
let permissionsMap = $state<Record<string, string[]>>({});
let isSaving = $state(false);

// Initialize or reset form when opened
$effect(() => {
  if (open) {
    if (editingRole) {
      roleId = editingRole.id;
      roleName = editingRole.name || "";
      roleNameAr = editingRole.name_ar || "";
      
      let sourcePerms = editingRole.permissions;

      if (sourcePerms?.["*"]?.includes("*") || editingRole.id === "super_admin") {
        const fullPerms: Record<string, string[]> = {};
        for (const domain of IAM_DOMAINS) {
          for (const res of domain.resources) {
            fullPerms[res.key] = res.actions.map((a) => a.key);
          }
        }
        permissionsMap = fullPerms;
      } else {
        const perms: Record<string, string[]> = {};
        if (sourcePerms) {
          for (const [k, v] of Object.entries(sourcePerms)) {
            perms[k] = [...v];
          }
        }
        permissionsMap = perms;
      }
    } else {
      roleId = "";
      roleName = "";
      roleNameAr = "";
      permissionsMap = {};
    }
  }
});

// Calculate statistics for each domain
function getDomainStat(domainId: string) {
  const domain = IAM_DOMAINS.find((d) => d.id === domainId);
  if (!domain) return { selected: 0, total: 0 };
  let total = 0;
  let selected = 0;
  for (const res of domain.resources) {
    total += res.actions.length;
    const currentResActions = permissionsMap[res.key] || [];
    selected += currentResActions.length;
  }
  return { selected, total };
}

// Toggle an action
function toggleAction(resourceKey: string, actionKey: string) {
  const current = permissionsMap[resourceKey] || [];
  if (current.includes(actionKey)) {
    const updated = current.filter((a) => a !== actionKey);
    if (updated.length === 0) {
      const next = { ...permissionsMap };
      delete next[resourceKey];
      permissionsMap = next;
    } else {
      permissionsMap = {
        ...permissionsMap,
        [resourceKey]: updated,
      };
    }
  } else {
    permissionsMap = {
      ...permissionsMap,
      [resourceKey]: [...current, actionKey],
    };
  }
}

// Toggle all actions for a resource
function toggleResourceAll(resource: IamResourceDef) {
  const current = permissionsMap[resource.key] || [];
  const allActionKeys = resource.actions.map((a) => a.key);

  if (current.length === allActionKeys.length) {
    const next = { ...permissionsMap };
    delete next[resource.key];
    permissionsMap = next;
  } else {
    permissionsMap = {
      ...permissionsMap,
      [resource.key]: [...allActionKeys],
    };
  }
}

// Submit handler
async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  const cleanId = roleId.toLowerCase().trim().replace(/[\s-]+/g, "_");
  if (!cleanId) {
    toast.error(m.ui_copy_role_id_is_required_98e11b83());
    return;
  }
  if (!roleName.trim() && !roleNameAr.trim()) {
    toast.error(m.ui_copy_role_name_is_required_27157b93());
    return;
  }

  isSaving = true;
  try {
    const payload = {
      id: cleanId,
      name: roleName.trim() || roleNameAr.trim(),
      name_ar: roleNameAr.trim() || undefined,
      permissions: permissionsMap,
    };

    const savedRole = await api.saveRole(payload);
    await queryClient.invalidateQueries({ queryKey: ["roles"] });

    toast.success(
      isEditMode
        ? m.ui_copy_role_updated_successfully_39fa3ad8()
        : m.ui_copy_role_created_successfully_e33128c9()
    );

    if (onsave) {
      await onsave(savedRole as RoleDto);
    }
    open = false;
  } catch (e: any) {
    toast.error(e?.message || m.ui_copy_failed_to_save_role_c9f36e45());
  } finally {
    isSaving = false;
  }
}
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-xl md:max-w-2xl overflow-hidden p-0 gap-0">
    <form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
      <!-- Sheet Header -->
      <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
        <SheetTitle class="text-lg font-bold text-foreground">
          {isEditMode
            ? m.ui_copy_edit_role_c8f33d66()
            : m.ui_copy_add_role_f3782222()}
        </SheetTitle>
      </SheetHeader>

      <!-- Sheet Body -->
      <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide text-start">
        <!-- Basic Info Fields -->
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
          <!-- Role ID / Code -->
          <FormField id="role_id_field" label={m.ui_copy_role_code_ca56677e()} required>
            {#snippet children()}
            <Input
              id="role_id_field"
              type="text"
              bind:value={roleId}
              disabled={isEditMode}
              placeholder="e.g. academic_lead"
              class="font-mono text-xs"
              required
            />
            {/snippet}
          </FormField>

          <!-- Arabic Name -->
          <FormField id="role_name_ar_field" label={m.common_org_unit_name_ar()}>
            {#snippet children()}
            <Input
              id="role_name_ar_field"
              type="text"
              bind:value={roleNameAr}
              placeholder="مثال: مشرف أكاديمي"
              class="text-xs"
            />
            {/snippet}
          </FormField>

          <!-- English Name -->
          <FormField id="role_name_en_field" label={m.common_org_unit_name_en()}>
            {#snippet children()}
            <Input
              id="role_name_en_field"
              type="text"
              bind:value={roleName}
              placeholder="e.g. Academic Lead"
              class="text-xs"
            />
            {/snippet}
          </FormField>
        </div>

        <!-- Permissions Accordion by Domain -->
        <div class="flex flex-col gap-2 pt-2">
          <span class="text-xs font-bold text-foreground">
            {m.ui_copy_domain_permissions_faa86b6f()}
          </span>

          <Accordion type="multiple" class="w-full">
            {#each IAM_DOMAINS as domain (domain.id)}
              {@const stat = getDomainStat(domain.id)}

              <AccordionItem
                value={domain.id}
                class="border border-border/60 rounded-xl px-4 py-1 mb-2 bg-card"
              >
                <AccordionTrigger class="text-xs font-semibold hover:no-underline py-2.5">
                  <div class="flex items-center justify-between gap-3 w-full pe-2">
                    <span>{translateIamMessage(domain.nameKey)}</span>
                    <Badge
                      variant={stat.selected > 0 ? "secondary" : "outline"}
                      class="text-[9px] font-mono px-1.5 py-0"
                    >
                      {stat.selected}/{stat.total}
                    </Badge>
                  </div>
                </AccordionTrigger>

                <AccordionContent class="pt-2 pb-3">
                  <div class="flex flex-col gap-4">
                    {#each domain.resources as res (res.key)}
                      {@const selectedActions = permissionsMap[res.key] || []}
                      {@const allSelected = selectedActions.length === res.actions.length}

                      <div class="p-3 rounded-lg border border-border/40 bg-muted/15 space-y-2.5">
                        <!-- Resource Title & Toggle All -->
                        <div class="flex items-center justify-between gap-2 border-b border-border/30 pb-2">
                          <div class="flex items-center gap-2">
                            <span class="text-xs font-bold text-foreground">
                              {translateIamMessage(res.labelKey)}
                            </span>
                            <span class="text-[10px] font-mono text-muted-foreground">
                              ({res.key})
                            </span>
                          </div>

                          <button
                            type="button"
                            onclick={() => toggleResourceAll(res)}
                            class="text-[10px] text-muted-foreground hover:text-foreground hover:underline cursor-pointer"
                          >
                            {allSelected
                              ? m.ui_copy_deselect_ccd71b49()
                              : m.ui_copy_select_all_99dda350()}
                          </button>
                        </div>

                        <!-- Action Checkboxes -->
                        <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
                          {#each res.actions as act (act.key)}
                            {@const isChecked = selectedActions.includes(act.key)}

                            <label
                              class="flex items-center gap-2 p-1.5 rounded-md hover:bg-muted/40 cursor-pointer select-none transition-colors"
                            >
                              <Checkbox
                                checked={isChecked}
                                onCheckedChange={() => toggleAction(res.key, act.key)}
                              />
                              <span class="text-xs text-foreground truncate">
                                {translateIamMessage(act.labelKey)}
                              </span>
                            </label>
                          {/each}
                        </div>
                      </div>
                    {/each}
                  </div>
                </AccordionContent>
              </AccordionItem>
            {/each}
          </Accordion>
        </div>
      </div>

      <!-- Sheet Footer Actions -->
      <div class="px-6 py-3 border-t border-border/40 flex gap-2 justify-end shrink-0 bg-background/50">
        <Button
          type="button"
          variant="ghost"
          onclick={() => (open = false)}
          disabled={isSaving}
          class="text-xs h-8"
        >
          {m.common_action_cancel()}
        </Button>
        <Button
          type="submit"
          variant="lime"
          class="font-semibold px-6 text-xs h-8"
          disabled={isSaving}
        >
          {isSaving
            ? "..."
            : (m.common_action_save())}
        </Button>
      </div>
    </form>
  </SheetContent>
</Sheet>
