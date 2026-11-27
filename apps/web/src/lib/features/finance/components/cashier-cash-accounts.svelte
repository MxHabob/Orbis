<script lang="ts">
import { useQueryClient } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui/button";
import { Label } from "@orbis/ui/label";
import { toast } from "@orbis/ui";
import { api, queryKeys, type AccountDto } from "$lib/api";
import * as m from "$lib/paraglide/messages";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import {
  createOrgSettingsQuery,
  createUsersQuery,
} from "$lib/features/settings/queries";
import { treasuryKind } from "../treasury-policy";

let { orgId, accounts }: { orgId: string; accounts: AccountDto[] } = $props();
const settingsQuery = createOrgSettingsQuery(() => orgId);
const usersQuery = createUsersQuery(() => orgId);
const queryClient = useQueryClient();
const id = $props.id();
let userId = $state<string | null>(null);
let draftAccountId = $state<string | null | undefined>(undefined);
let saving = $state(false);
const users = $derived(
  (usersQuery.data ?? []).filter(
    (user) => user.org_id === orgId && user.is_active,
  ),
);
const cashboxes = $derived(
  accounts.filter((account) => treasuryKind(account) === "cash"),
);
const assignedAccountId = $derived(
  userId
    ? settingsQuery.data?.finance?.cashier_cash_accounts?.[userId]
    : undefined,
);
const selectedAccountId = $derived(
  draftAccountId === undefined ? (assignedAccountId ?? null) : draftAccountId,
);
const loading = $derived(settingsQuery.isPending || usersQuery.isPending);
const loadFailed = $derived(settingsQuery.isError || usersQuery.isError);

async function save() {
  const cashierId = userId;
  const accountId = draftAccountId;
  if (!cashierId || accountId === undefined || saving || loadFailed) return;
  if (accountId && !cashboxes.some((account) => account.id === accountId))
    return;
  saving = true;
  try {
    const settings = await api.getSettings();
    if (!settings.finance) throw new Error("Missing finance settings");
    const assignments = { ...settings.finance.cashier_cash_accounts };
    if (accountId) assignments[cashierId] = accountId;
    else delete assignments[cashierId];
    const updated = await api.updateSettings({
      ...settings,
      finance: { ...settings.finance, cashier_cash_accounts: assignments },
    });
    queryClient.setQueryData(queryKeys.org.settings(orgId), updated);
    draftAccountId = undefined;
    toast.success(m.finance_ui_cashier_assignment_saved());
  } catch {
    toast.error(m.finance_ui_cashier_assignment_failed());
  } finally {
    saving = false;
  }
}
</script>

<section class="space-y-3 border-b border-border px-4 py-3">
  <div>
    <h3 class="text-sm font-semibold">{m.finance_ui_cashier_assignment_title()}</h3>
    <p class="text-xs text-muted-foreground">{m.finance_ui_cashier_assignment_hint()}</p>
  </div>
  {#if loadFailed}
    <div class="flex items-center gap-2 text-sm" role="status">
      {m.finance_ui_cashier_assignment_load_failed()}
      <Button variant="ghost" size="sm" onclick={() => { void settingsQuery.refetch(); void usersQuery.refetch(); }}>
        {m.common_retry()}
      </Button>
    </div>
  {:else}
    <div class="grid gap-3 sm:grid-cols-[minmax(0,1fr)_minmax(0,1fr)_auto] sm:items-end">
      <div class="space-y-1">
        <Label for={id + "-user"}>{m.finance_ui_cashier_assignment_user()}</Label>
        <SearchableSelect
          id={id + "-user"}
          items={users}
          value={userId}
          placeholder={m.finance_ui_cashier_assignment_user()}
          getItemLabel={(user) => user.full_name_ar || user.full_name}
          getItemSubtitle={(user) => user.email ?? undefined}
          disabled={saving || loading}
          onchange={(value) => { userId = value; draftAccountId = undefined; }}
        />
      </div>
      <div class="space-y-1">
        <Label for={id + "-box"}>{m.finance_ui_cash_treasury()}</Label>
        <SearchableSelect
          id={id + "-box"}
          items={cashboxes}
          value={selectedAccountId}
          placeholder={m.finance_ui_cashier_assignment_unrestricted()}
          disabled={saving || loading || !userId}
          onchange={(value) => { draftAccountId = value; }}
        />
      </div>
      <div class="flex gap-2">
        {#if selectedAccountId}
          <Button variant="ghost" disabled={saving || loading} onclick={() => { draftAccountId = null; }}>
            {m.finance_ui_cashier_assignment_remove()}
          </Button>
        {/if}
        <Button disabled={saving || loading || !userId || draftAccountId === undefined || draftAccountId === (assignedAccountId ?? null)} onclick={save}>
          {saving ? m.common_loading() : m.common_save_changes()}
        </Button>
      </div>
    </div>
  {/if}
</section>
