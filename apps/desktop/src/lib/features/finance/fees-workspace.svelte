<!--
@component FeesWorkspace
Lists all student fees across the school, showing descriptions, student names,
due dates, ledger accounts, status indicators, and total amounts.
Allows triggering FeeAssignSheet to bill a student.
-->
<script lang="ts">
  import { api } from "$lib/api";
  import { studentFeeFilterFields } from "$lib/core/filter-configs";
  import { formatCurrency, formatDate, l } from "$lib/core/i18n";
  import FeeAssignSheet from "$lib/features/finance/components/fee-assign-sheet.svelte";
  import * as m from "$lib/paraglide/messages";
  import { getLocale } from "$lib/paraglide/runtime";
  import {
    CheckSquare,
    Document,
    Plus,
    QuestionMarkCircle,
    Refresh,
    Search,
  } from "@orbis/icons";
  import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
  import { Button } from "@orbis/ui/button";
  import {
    FilterTrigger,
    type UniversalFilter,
  } from "@orbis/ui/filter-builder";
  import { Input } from "@orbis/ui/input";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  let { orgId }: { orgId: string } = $props();

  // State
  let fees = $state<any[]>([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);

  // Filters State
  let searchQuery = $state("");
  let isSheetOpen = $state(false);

  async function loadFees() {
    isLoading = true;
    error = null;
    try {
      const list = await api.listStudentFees();
      fees = list;
    } catch (e) {
      console.error("Error loading fees:", e);
      error = e instanceof Error ? e.message : String(e);
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    if (orgId) {
      loadFees();
    }
  });

  let activeFilter = $state<UniversalFilter>({
    logic: "AND",
    conditions: [],
    groups: [],
  });

  // Calculations
  const stats = $derived.by(() => {
    let invoiced = 0;
    let collected = 0;
    for (const f of fees) {
      invoiced += Number(f.amount || 0);
      collected += Number(f.paid_amount || 0);
    }
    return {
      invoiced,
      collected,
      due: Math.max(0, invoiced - collected),
    };
  });

  function getStatusLabel(statusStr: string) {
    const s = String(statusStr).toLowerCase();
    if (s === "paid") return m.finance_fees_status_paid();
    if (s === "partially_paid") return m.finance_fees_status_partially_paid();
    return m.finance_fees_status_unpaid();
  }

  // Grid Source
  const gridSource = $derived(
    fees.map((f, i) => ({
      ...f,
      __idx: i + 1,
      student_name: f.student_name || f.enrollment_id || "—",
      student_avatar: f.student_avatar || f.avatar || null,
      template_name: f.description || m.common_ft_category_tuition(),
      total_amount: formatCurrency(Number(f.amount || 0), f.currency || "YER"),
      paid_amount: formatCurrency(
        Number(f.paid_amount || 0),
        f.currency || "YER",
      ),
      balance: formatCurrency(
        Math.max(0, Number(f.amount || 0) - Number(f.paid_amount || 0)),
        f.currency || "YER",
      ),
      payment_status: getStatusLabel(f.status || "unpaid"),
      created_at_display: f.created_at
        ? formatDate(f.created_at, "datetime")
        : "—",
      updated_at_display: f.updated_at
        ? formatDate(f.updated_at, "datetime")
        : "—",
    })),
  );

  // Columns
  const columns: OrbisColumnDef[] = [
    {
      id: "__idx",
      prop: "__idx",
      header: "#",
      name: "#",
      width: 48,
      size: 48,
      readonly: true,
      pin: "left",
      align: "center",
    },
    {
      id: "student_name",
      prop: "student_name",
      header: m.common_word_student(),
      name: m.common_word_student(),
      width: 220,
      size: 220,
      type: "avatar",
      readonly: true,
      pin: "left",
    },
    {
      id: "template_name",
      prop: "template_name",
      header: m.finance_fees_col_description(),
      name: m.finance_fees_col_description(),
      width: 180,
      size: 180,
      readonly: true,
    },
    {
      id: "total_amount",
      prop: "total_amount",
      header: m.common_ft_col_amount(),
      name: m.common_ft_col_amount(),
      width: 120,
      size: 120,
      readonly: true,
      align: "end",
      cellClass: "font-mono font-medium",
    },
    {
      id: "paid_amount",
      prop: "paid_amount",
      header: m.finance_fees_total_collected(),
      name: m.finance_fees_total_collected(),
      width: 120,
      size: 120,
      readonly: true,
      align: "end",
      cellClass: "font-mono text-emerald-600 font-medium",
    },
    {
      id: "balance",
      prop: "balance",
      header: m.finance_fees_total_due(),
      name: m.finance_fees_total_due(),
      width: 120,
      size: 120,
      readonly: true,
      align: "end",
      cellClass: "font-mono text-rose-600 font-bold",
    },
    {
      id: "payment_status",
      prop: "payment_status",
      header: m.finance_fees_col_status(),
      name: m.finance_fees_col_status(),
      width: 110,
      size: 110,
      type: "badge",
      align: "center",
      readonly: true,
    },
    {
      id: "created_at_display",
      prop: "created_at_display",
      header: l("تاريخ الإنشاء", "Created At"),
      name: "Created At",
      width: 130,
      size: 130,
      readonly: true,
      cellClass: "font-mono text-xs text-muted-foreground",
    },
    {
      id: "updated_at_display",
      prop: "updated_at_display",
      header: l("آخر تحديث", "Updated At"),
      name: "Updated At",
      width: 130,
      size: 130,
      readonly: true,
      cellClass: "font-mono text-xs text-muted-foreground",
    },
  ];

  const rtl = $derived(getLocale() === "ar");
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-5 font-sans">
  <div class="flex items-center justify-between gap-4 shrink-0 select-none">
    <div class="relative flex-1 max-w-xs">
      <Search
        size={14}
        class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
      />
      <Input
        type="text"
        placeholder={m.finance_fees_search_placeholder()}
        bind:value={searchQuery}
        class="h-8 ps-8 text-xs bg-card border-border/60"
      />
    </div>

    <div class="flex items-center gap-2 shrink-0">
      <FilterTrigger
        bind:filter={activeFilter}
        fields={studentFeeFilterFields}
      />
      <Button
        variant="ghost"
        size="icon-sm"
        onclick={loadFees}
        class="text-muted-foreground"
      >
        <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
      </Button>
      <Button
        variant="lime"
        class="text-xs h-8 px-3 shrink-0"
        onclick={() => (isSheetOpen = true)}
      >
        <Plus size={14} class="me-1.5" />
        {m.finance_fees_assign_button()}
      </Button>
    </div>
  </div>

  <!-- Statistics summary row -->
  {#if !isLoading && fees.length > 0}
    <div
      class="grid grid-cols-1 sm:grid-cols-3 gap-4"
      in:fade={{ duration: 150 }}
    >
      <!-- Total Invoiced -->
      <div
        class="bg-card border border-border rounded-xl flex items-center justify-between p-4 h-20 select-none"
      >
        <div class="flex flex-col gap-1 text-start">
          <span
            class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider"
          >
            {m.finance_fees_total_invoiced()}
          </span>
          <span class="text-base font-bold text-foreground font-mono"
            >{formatCurrency(stats.invoiced, "YER")}</span
          >
        </div>
        <div
          class="h-8 w-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary"
        >
          <Document size={15} />
        </div>
      </div>

      <!-- Total Collected -->
      <div
        class="bg-card border border-border rounded-xl flex items-center justify-between p-4 h-20 select-none"
      >
        <div class="flex flex-col gap-1 text-start">
          <span
            class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider"
          >
            {m.finance_fees_total_collected()}
          </span>
          <span class="text-base font-bold text-emerald-500 font-mono"
            >{formatCurrency(stats.collected, "YER")}</span
          >
        </div>
        <div
          class="h-8 w-8 rounded-lg bg-emerald-500/10 flex items-center justify-center text-emerald-500"
        >
          <CheckSquare size={14} />
        </div>
      </div>

      <!-- Balance Due -->
      <div
        class="bg-card border border-border rounded-xl flex items-center justify-between p-4 h-20 select-none"
      >
        <div class="flex flex-col gap-1 text-start">
          <span
            class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider"
          >
            {m.finance_fees_total_due()}
          </span>
          <span class="text-base font-bold text-rose-500 font-mono"
            >{formatCurrency(stats.due, "YER")}</span
          >
        </div>
        <div
          class="h-8 w-8 rounded-lg bg-rose-500/10 flex items-center justify-center text-rose-500"
        >
          <QuestionMarkCircle size={15} />
        </div>
      </div>
    </div>
  {/if}

  <!-- Main Billing Grid -->
  <div class="flex-1 min-h-0">
    <OrbisGrid
      {columns}
      source={gridSource}
      loading={isLoading}
      clientQuery
      search={searchQuery}
      filter={activeFilter}
      {rtl}
      readonly
      emptyTitle={m.finance_fees_empty_title()}
      emptyDescription={m.finance_fees_empty_desc()}
    />
  </div>
</div>

<!-- Modal Form Dialog -->
<FeeAssignSheet bind:open={isSheetOpen} {orgId} onSuccess={loadFees} />
