<!--
  Overview / Dashboard page.
  Uses the MainViewport split-pane layout.
  Left: Quick stats | Middle: Main content area | Right: Activity feed
-->
<script lang="ts">
	import MainViewport from '$lib/components/layout/MainViewport.svelte';
	import { Users, BookOpen, Wallet, TrendingUp, Clock, CalendarDays } from 'lucide-svelte';
	import type { LayoutData } from './$types';

	let { data }: { data: LayoutData } = $props();
</script>

<MainViewport
	name="overview"
	leftWidth={{ default: 260, min: 220 }}
	rightWidth={{ default: 280, min: 220 }}
>
	{#snippet left()}
		<div class="stats-panel">
			<div class="stats-header">
				<h2 class="stats-title">Quick Stats</h2>
				<span class="stats-badge">Today</span>
			</div>

			<div class="stats-grid">
				<div class="stat-card">
					<div class="stat-icon" style="--clr: hsl(var(--primary))">
						<Users size={16} strokeWidth={2} />
					</div>
					<div class="stat-info">
						<span class="stat-value">1,247</span>
						<span class="stat-label">Total Students</span>
					</div>
				</div>

				<div class="stat-card">
					<div class="stat-icon" style="--clr: #22c55e">
						<TrendingUp size={16} strokeWidth={2} />
					</div>
					<div class="stat-info">
						<span class="stat-value">94.2%</span>
						<span class="stat-label">Attendance Rate</span>
					</div>
				</div>

				<div class="stat-card">
					<div class="stat-icon" style="--clr: #f59e0b">
						<BookOpen size={16} strokeWidth={2} />
					</div>
					<div class="stat-info">
						<span class="stat-value">42</span>
						<span class="stat-label">Active Classes</span>
					</div>
				</div>

				<div class="stat-card">
					<div class="stat-icon" style="--clr: #8b5cf6">
						<Wallet size={16} strokeWidth={2} />
					</div>
					<div class="stat-info">
						<span class="stat-value">78.5%</span>
						<span class="stat-label">Fees Collected</span>
					</div>
				</div>
			</div>

			<div class="upcoming-section">
				<h3 class="section-label">Upcoming</h3>
				{#each ['Parent Meeting — Tomorrow', 'Mid-term Exams — Nov 15', 'Winter Break — Dec 20'] as event, i}
					<div class="upcoming-item">
						<CalendarDays size={13} strokeWidth={2} class="upcoming-icon" />
						<span class="upcoming-text">{event}</span>
					</div>
				{/each}
			</div>
		</div>
	{/snippet}

	{#snippet middle()}
		<div class="main-content">
			<div class="welcome-section animate-fly-in">
				<h1 class="welcome-title">Welcome back</h1>
				<p class="welcome-subtitle">Here's what's happening at your school today.</p>
			</div>

			<div class="widgets-grid animate-fly-in" style="animation-delay: 0.05s">
				<div class="widget-card widget-wide">
					<div class="widget-header">
						<span class="widget-title">Attendance Overview</span>
						<span class="widget-period">This Week</span>
					</div>
					<div class="widget-body chart-placeholder dotted-pattern">
						<span class="placeholder-text">Attendance Chart</span>
					</div>
				</div>

				<div class="widget-card">
					<div class="widget-header">
						<span class="widget-title">Grade Distribution</span>
					</div>
					<div class="widget-body chart-placeholder dotted-pattern">
						<span class="placeholder-text">Grade Chart</span>
					</div>
				</div>

				<div class="widget-card">
					<div class="widget-header">
						<span class="widget-title">Fee Collection</span>
					</div>
					<div class="widget-body chart-placeholder dotted-pattern">
						<span class="placeholder-text">Fee Chart</span>
					</div>
				</div>
			</div>
		</div>
	{/snippet}

	{#snippet right()}
		<div class="activity-panel">
			<h2 class="activity-title">Recent Activity</h2>

			<div class="activity-list">
				{#each [
					{ time: '2m ago', text: 'Ahmed was marked absent in Math' },
					{ time: '15m ago', text: 'Fee payment received — $450' },
					{ time: '1h ago', text: 'New student enrolled: Sara Ali' },
					{ time: '2h ago', text: 'Grade report published for Grade 10' },
					{ time: '3h ago', text: 'Schedule updated for next week' },
					{ time: '5h ago', text: 'Parent meeting notes uploaded' },
				] as item}
					<div class="activity-item">
						<div class="activity-dot"></div>
						<div class="activity-content">
							<span class="activity-text">{item.text}</span>
							<span class="activity-time">
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

<style>
	/* ── Stats Panel (Left) ── */
	.stats-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		padding: 16px;
		gap: 16px;
		background-color: var(--panel-bg);
	}

	.stats-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.stats-title {
		font-size: 13px;
		font-weight: 600;
		color: hsl(var(--foreground));
	}

	.stats-badge {
		font-size: 10px;
		font-weight: 600;
		padding: 2px 8px;
		border-radius: 20px;
		background: hsl(var(--primary) / 0.1);
		color: hsl(var(--primary));
	}

	.stats-grid {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 10px;
		border-radius: var(--radius-m);
		background: hsl(var(--muted) / 0.5);
		transition: background-color var(--transition-fast);
	}

	.stat-card:hover {
		background: hsl(var(--muted));
	}

	.stat-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border-radius: var(--radius-m);
		background: color-mix(in srgb, var(--clr) 12%, transparent);
		color: var(--clr);
		flex-shrink: 0;
	}

	.stat-info {
		display: flex;
		flex-direction: column;
		gap: 1px;
	}

	.stat-value {
		font-size: 14px;
		font-weight: 700;
		color: hsl(var(--foreground));
		line-height: 1;
	}

	.stat-label {
		font-size: 11px;
		color: hsl(var(--muted-foreground));
	}

	.upcoming-section {
		margin-top: auto;
	}

	.section-label {
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: hsl(var(--muted-foreground));
		margin-bottom: 8px;
	}

	.upcoming-item {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 6px 0;
		font-size: 12px;
		color: hsl(var(--foreground));
	}

	:global(.upcoming-icon) {
		color: hsl(var(--muted-foreground));
		flex-shrink: 0;
	}

	.upcoming-text {
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* ── Main Content (Middle) ── */
	.main-content {
		display: flex;
		flex-direction: column;
		height: 100%;
		padding: 20px;
		gap: 20px;
		overflow-y: auto;
		background-color: var(--panel-bg);
	}

	.welcome-section {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.welcome-title {
		font-size: 20px;
		font-weight: 700;
		color: hsl(var(--foreground));
	}

	.welcome-subtitle {
		font-size: 13px;
		color: hsl(var(--muted-foreground));
	}

	.widgets-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 12px;
	}

	.widget-card {
		display: flex;
		flex-direction: column;
		border: 1px solid var(--panel-border);
		border-radius: var(--radius-ml);
		overflow: hidden;
	}

	.widget-wide {
		grid-column: span 2;
	}

	.widget-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px 14px;
		border-bottom: 1px solid var(--panel-border);
	}

	.widget-title {
		font-size: 12px;
		font-weight: 600;
		color: hsl(var(--foreground));
	}

	.widget-period {
		font-size: 11px;
		color: hsl(var(--muted-foreground));
	}

	.widget-body {
		padding: 16px;
		min-height: 120px;
	}

	.chart-placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.placeholder-text {
		font-size: 12px;
		color: hsl(var(--muted-foreground));
		opacity: 0.5;
	}

	/* ── Activity Panel (Right) ── */
	.activity-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		padding: 16px;
		gap: 12px;
		background-color: var(--panel-bg);
		overflow-y: auto;
	}

	.activity-title {
		font-size: 13px;
		font-weight: 600;
		color: hsl(var(--foreground));
	}

	.activity-list {
		display: flex;
		flex-direction: column;
	}

	.activity-item {
		display: flex;
		gap: 10px;
		padding: 10px 0;
		border-bottom: 1px solid hsl(var(--border) / 0.5);
	}

	.activity-item:last-child {
		border-bottom: none;
	}

	.activity-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		background-color: hsl(var(--primary));
		margin-top: 5px;
		flex-shrink: 0;
	}

	.activity-content {
		display: flex;
		flex-direction: column;
		gap: 3px;
		min-width: 0;
	}

	.activity-text {
		font-size: 12px;
		color: hsl(var(--foreground));
		line-height: 1.4;
	}

	.activity-time {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 10px;
		color: hsl(var(--muted-foreground));
	}
</style>
