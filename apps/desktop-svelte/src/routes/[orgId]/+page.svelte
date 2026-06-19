<!--
  Overview / Dashboard page.
  Uses the MainViewport split-pane layout.
  Styled completely with @orbis/ui-svelte components and Tailwind CSS.
-->
<script lang="ts">
	import MainViewport from '$lib/components/layout/MainViewport.svelte';
	import { Users, BookOpen, Wallet, TrendingUp, Clock, CalendarDays } from 'lucide-svelte';
	import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@orbis/ui-svelte/card';
	import { Badge } from '@orbis/ui-svelte/badge';
	import type { LayoutData } from './$types';

	let { data }: { data: LayoutData } = $props();
</script>

<MainViewport
	name="overview"
	leftWidth={{ default: 260, min: 220 }}
	rightWidth={{ default: 280, min: 220 }}
>
	{#snippet left()}
		<div class="flex flex-col h-full p-4 gap-4 bg-background">
			<div class="flex items-center justify-between">
				<h2 class="text-xs font-semibold text-foreground">Quick Stats</h2>
				<Badge class="bg-primary/10 text-primary border-transparent text-[10px] font-semibold px-2 py-0.5">Today</Badge>
			</div>

			<div class="flex flex-col gap-2">
				<Card class="flex flex-row items-center gap-3 p-3 bg-muted/40 hover:bg-muted/70 transition-colors border-transparent cursor-pointer rounded-xl" size="sm">
					<div class="flex h-8 w-8 items-center justify-center rounded-xl bg-primary/10 text-primary shrink-0">
						<Users size={16} strokeWidth={2} />
					</div>
					<div class="flex flex-col gap-0.5">
						<span class="text-sm font-bold text-foreground leading-none">1,247</span>
						<span class="text-[10px] text-muted-foreground">Total Students</span>
					</div>
				</Card>

				<Card class="flex flex-row items-center gap-3 p-3 bg-muted/40 hover:bg-muted/70 transition-colors border-transparent cursor-pointer rounded-xl" size="sm">
					<div class="flex h-8 w-8 items-center justify-center rounded-xl bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 shrink-0">
						<TrendingUp size={16} strokeWidth={2} />
					</div>
					<div class="flex flex-col gap-0.5">
						<span class="text-sm font-bold text-foreground leading-none">94.2%</span>
						<span class="text-[10px] text-muted-foreground">Attendance Rate</span>
					</div>
				</Card>

				<Card class="flex flex-row items-center gap-3 p-3 bg-muted/40 hover:bg-muted/70 transition-colors border-transparent cursor-pointer rounded-xl" size="sm">
					<div class="flex h-8 w-8 items-center justify-center rounded-xl bg-amber-500/10 text-amber-600 dark:text-amber-400 shrink-0">
						<BookOpen size={16} strokeWidth={2} />
					</div>
					<div class="flex flex-col gap-0.5">
						<span class="text-sm font-bold text-foreground leading-none">42</span>
						<span class="text-[10px] text-muted-foreground">Active Classes</span>
					</div>
				</Card>

				<Card class="flex flex-row items-center gap-3 p-3 bg-muted/40 hover:bg-muted/70 transition-colors border-transparent cursor-pointer rounded-xl" size="sm">
					<div class="flex h-8 w-8 items-center justify-center rounded-xl bg-violet-500/10 text-violet-600 dark:text-violet-400 shrink-0">
						<Wallet size={16} strokeWidth={2} />
					</div>
					<div class="flex flex-col gap-0.5">
						<span class="text-sm font-bold text-foreground leading-none">78.5%</span>
						<span class="text-[10px] text-muted-foreground">Fees Collected</span>
					</div>
				</Card>
			</div>

			<div class="mt-auto">
				<h3 class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground mb-2">Upcoming</h3>
				<div class="flex flex-col gap-1">
					{#each ['Parent Meeting — Tomorrow', 'Mid-term Exams — Nov 15', 'Winter Break — Dec 20'] as event, i}
						<div class="flex items-center gap-2 py-1 text-xs text-foreground truncate">
							<CalendarDays size={13} strokeWidth={2} class="text-muted-foreground shrink-0" />
							<span class="truncate">{event}</span>
						</div>
					{/each}
				</div>
			</div>
		</div>
	{/snippet}

	{#snippet middle()}
		<div class="flex flex-col h-full p-5 gap-5 overflow-y-auto bg-background">
			<div class="flex flex-col gap-1">
				<h1 class="text-xl font-bold text-foreground">Welcome back</h1>
				<p class="text-xs text-muted-foreground">Here's what's happening at your school today.</p>
			</div>

			<div class="grid grid-cols-2 gap-3">
				<Card class="col-span-2 flex flex-col border border-border shadow-sm rounded-xl overflow-hidden bg-card">
					<CardHeader class="flex flex-row items-center justify-between p-3.5 border-b border-border bg-muted/20">
						<CardTitle class="text-xs font-semibold text-foreground">Attendance Overview</CardTitle>
						<CardDescription class="text-[10px] text-muted-foreground">This Week</CardDescription>
					</CardHeader>
					<CardContent class="p-4 min-h-[140px] flex items-center justify-center dotted-pattern">
						<span class="text-xs text-muted-foreground/50 font-medium">Attendance Chart</span>
					</CardContent>
				</Card>

				<Card class="flex flex-col border border-border shadow-sm rounded-xl overflow-hidden bg-card">
					<CardHeader class="flex flex-row items-center justify-between p-3.5 border-b border-border bg-muted/20">
						<CardTitle class="text-xs font-semibold text-foreground">Grade Distribution</CardTitle>
					</CardHeader>
					<CardContent class="p-4 min-h-[140px] flex items-center justify-center dotted-pattern">
						<span class="text-xs text-muted-foreground/50 font-medium">Grade Chart</span>
					</CardContent>
				</Card>

				<Card class="flex flex-col border border-border shadow-sm rounded-xl overflow-hidden bg-card">
					<CardHeader class="flex flex-row items-center justify-between p-3.5 border-b border-border bg-muted/20">
						<CardTitle class="text-xs font-semibold text-foreground">Fee Collection</CardTitle>
					</CardHeader>
					<CardContent class="p-4 min-h-[140px] flex items-center justify-center dotted-pattern">
						<span class="text-xs text-muted-foreground/50 font-medium">Fee Chart</span>
					</CardContent>
				</Card>
			</div>
		</div>
	{/snippet}

	{#snippet right()}
		<div class="flex flex-col h-full p-4 gap-3 bg-background overflow-y-auto">
			<h2 class="text-xs font-semibold text-foreground">Recent Activity</h2>

			<div class="flex flex-col">
				{#each [
					{ time: '2m ago', text: 'Ahmed was marked absent in Math' },
					{ time: '15m ago', text: 'Fee payment received — $450' },
					{ time: '1h ago', text: 'New student enrolled: Sara Ali' },
					{ time: '2h ago', text: 'Grade report published for Grade 10' },
					{ time: '3h ago', text: 'Schedule updated for next week' },
					{ time: '5h ago', text: 'Parent meeting notes uploaded' },
				] as item}
					<div class="flex gap-2.5 py-3 border-b border-border/40 last:border-b-0">
						<div class="w-1.5 h-1.5 rounded-full bg-primary mt-1.5 shrink-0"></div>
						<div class="flex flex-col gap-1 min-w-0">
							<span class="text-xs text-foreground leading-normal">{item.text}</span>
							<span class="flex items-center gap-1 text-[9px] text-muted-foreground">
								<Clock size={10} strokeWidth={2} />
								{item.time}
							</span>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/snippet}
</MainViewport>
