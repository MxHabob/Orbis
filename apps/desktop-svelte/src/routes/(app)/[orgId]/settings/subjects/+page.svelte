<!--
@component AcademicsSubjectsPage
Lists all academic subjects with themed visual icons, credit hours,
and triggers the subject creation side sheet.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Plus,
  BookOpen,
  Calculator,
  FlaskConical,
  Languages,
  Atom,
  Palette,
  Heart,
  Award,
  Search,
} from "lucide-svelte";
import { toast } from "svelte-sonner";
import { fade } from "svelte/transition";
import SearchField from "$lib/components/shared/search-field.svelte";
import SubjectCreateSheet from "$lib/components/sheets/subject-create-sheet.svelte";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State
let subjects = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);
let searchQuery = $state("");

// Sheet trigger state
let isModalOpen = $state(false);

// Load data
async function loadData() {
  isLoading = true;
  error = null;
  try {
    const list = await commands.subjects.getAll(data.orgId);
    subjects = list;
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
    toast.error(m.subjects_toast_load_error());
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadData();
  }
});

// Derived filtered subjects
const filteredSubjects = $derived(
  subjects.filter((sub) => {
    if (!searchQuery) return true;
    const query = searchQuery.toLowerCase().trim();
    return (
      String(sub.name || "")
        .toLowerCase()
        .includes(query) ||
      String(sub.name_ar || "")
        .toLowerCase()
        .includes(query) ||
      String(sub.code || "")
        .toLowerCase()
        .includes(query)
    );
  }),
);

// Map subjects to a themed icon and class
function getSubjectTheme(nameStr: string) {
  const nameLower = nameStr.toLowerCase();
  if (
    nameLower.includes("math") ||
    nameLower.includes("رياض") ||
    nameLower.includes("حساب")
  ) {
    return {
      icon: Calculator,
      bg: "bg-blue-500/10 text-blue-500 border-blue-500/20",
    };
  }
  if (
    nameLower.includes("science") ||
    nameLower.includes("علوم") ||
    nameLower.includes("phys") ||
    nameLower.includes("chem")
  ) {
    return {
      icon: FlaskConical,
      bg: "bg-emerald-500/10 text-emerald-500 border-emerald-500/20",
    };
  }
  if (
    nameLower.includes("english") ||
    nameLower.includes("انجليزي") ||
    nameLower.includes("lang") ||
    nameLower.includes("فرنسي") ||
    nameLower.includes("french")
  ) {
    return {
      icon: Languages,
      bg: "bg-indigo-500/10 text-indigo-500 border-indigo-500/20",
    };
  }
  if (
    nameLower.includes("islam") ||
    nameLower.includes("دين") ||
    nameLower.includes("قرآن") ||
    nameLower.includes("تربية اسلامية")
  ) {
    return {
      icon: Heart,
      bg: "bg-amber-500/10 text-amber-500 border-amber-500/20",
    };
  }
  if (
    nameLower.includes("art") ||
    nameLower.includes("فنية") ||
    nameLower.includes("رسم")
  ) {
    return {
      icon: Palette,
      bg: "bg-pink-500/10 text-pink-500 border-pink-500/20",
    };
  }
  if (
    nameLower.includes("computer") ||
    nameLower.includes("حاسب") ||
    nameLower.includes("تقنية")
  ) {
    return {
      icon: Atom,
      bg: "bg-cyan-500/10 text-cyan-500 border-cyan-500/20",
    };
  }
  return { icon: BookOpen, bg: "bg-primary/10 text-primary border-primary/20" };
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6">
	<!-- Header & Search Controls -->
	<div class="flex items-center justify-between gap-4 font-sans border-b border-border/40 pb-5 select-none">
		<!-- Search -->
		<SearchField
			placeholder={m.settings_subjects_search_placeholder_custom()}
			bind:value={searchQuery}
			class="max-w-md bg-background/50"
		/>

		<!-- Add button -->
		<Button variant="lime" class="text-xs h-9 px-4 font-semibold shrink-0" onclick={() => isModalOpen = true}>
			<Plus size={14} class="me-1.5" />
			{m.subjects_add_subject()}
		</Button>
	</div>

	<!-- Main Subject View Content -->
	{#if isLoading}
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
			<Skeleton class="h-28 w-full" />
			<Skeleton class="h-28 w-full" />
			<Skeleton class="h-28 w-full" />
			<Skeleton class="h-28 w-full" />
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-center gap-3">
			<p class="text-xs text-destructive font-semibold">{m.subjects_load_error()}</p>
			<p class="text-xs text-muted-foreground">{error}</p>
			<Button variant="outline" class="text-xs" onclick={loadData}>{m.error_retry()}</Button>
		</div>
	{:else if filteredSubjects.length === 0}
		<div class="flex flex-col items-center justify-center py-16 text-muted-foreground border border-border border-dashed rounded-xl gap-4">
			<BookOpen size={48} class="text-muted-foreground/30" />
			<div class="flex flex-col gap-1 text-center">
				<p class="text-xs font-semibold">{m.subjects_empty_title()}</p>
				<p class="text-[10px] text-muted-foreground/80">
					{searchQuery ? m.subjects_empty_desc_search() : m.subjects_empty_desc_general()}
				</p>
			</div>
			{#if !searchQuery}
				<Button variant="lime" class="text-xs mt-2" onclick={() => isModalOpen = true}>
					{m.subjects_add_subject()}
				</Button>
			{/if}
		</div>
	{:else}
		<!-- Subjects Grid -->
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4" in:fade={{ duration: 200 }}>
			{#each filteredSubjects as sub (sub.id)}
				{@const theme = getSubjectTheme(sub.name || sub.name_ar || "")}
				<div class="card-tactile bg-card flex items-start gap-4 p-4 h-28 select-none group relative transition-all duration-150">
					<!-- Subject Icon Block -->
					<div class="h-12 w-12 rounded-lg border flex items-center justify-center shrink-0 transition-transform duration-200 group-hover:scale-105 {theme.bg}">
						<theme.icon size={22} />
					</div>

					<!-- Subject Text Info -->
					<div class="flex flex-col text-start justify-between h-full flex-1 min-w-0">
						<div class="flex flex-col">
							<span class="text-xs font-bold text-foreground truncate group-hover:text-primary transition-colors">
								{sub.name_ar || sub.name || sub.id}
							</span>
							{#if sub.name && sub.name_ar}
								<span class="text-[9px] text-muted-foreground font-mono truncate">{sub.name}</span>
							{/if}
						</div>

						<div class="flex items-center justify-between text-[9px] text-muted-foreground mt-1.5 border-t border-border/40 pt-1.5">
							<span class="px-1.5 py-0.2 rounded border border-border bg-background font-mono">
								{sub.code || `SUB-${String(sub.id).slice(0, 3).toUpperCase()}`}
							</span>
							<span class="flex items-center gap-1">
								<Award size={9} />
								<span>{m.subjects_periods_count({ count: sub.credit_hours || 3 })}</span>
							</span>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<!-- Subject Creation Side Sheet Component -->
<SubjectCreateSheet bind:open={isModalOpen} orgId={data.orgId} onSuccess={loadData} />
