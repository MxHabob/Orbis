<!--
  Organization Bootstrap Screen — create the first organization and admin user.
  This screen appears after database connection when no organization exists.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { api } from "$lib/api/client";
import { authStore } from "$lib/stores";
import { Loader2, Building2, User, ArrowRight, ArrowLeft } from "@lucide/svelte";

// Form state
let orgName = $state("");
let orgNameAr = $state("");
let orgType = $state("school");
let timezone = $state("Asia/Riyadh");
let locale = $state("ar");
let currency = $state("SAR");
let adminFullName = $state("");
let adminEmail = $state("");
let adminPassword = $state("");
let adminPasswordConfirm = $state("");

let step = $state(1); // 1 = org info, 2 = admin info
let isSubmitting = $state(false);
let error = $state("");

const orgTypes = [
  { value: "school", label: "School", labelAr: "مدرسة" },
  { value: "university", label: "University", labelAr: "جامعة" },
  { value: "institute", label: "Training Institute", labelAr: "معهد تدريب" },
];

const currencies = [
  { code: "SAR", label: "Saudi Riyal (SAR)" },
  { code: "USD", label: "US Dollar (USD)" },
  { code: "EUR", label: "Euro (EUR)" },
  { code: "GBP", label: "British Pound (GBP)" },
  { code: "AED", label: "UAE Dirham (AED)" },
  { code: "EGP", label: "Egyptian Pound (EGP)" },
  { code: "KWD", label: "Kuwaiti Dinar (KWD)" },
  { code: "QAR", label: "Qatari Riyal (QAR)" },
  { code: "BHD", label: "Bahraini Dinar (BHD)" },
  { code: "OMR", label: "Omani Rial (OMR)" },
  { code: "JOD", label: "Jordanian Dinar (JOD)" },
];

const timezones = [
  { value: "Asia/Riyadh", label: "Riyadh (UTC+3)" },
  { value: "Asia/Dubai", label: "Dubai (UTC+4)" },
  { value: "Africa/Cairo", label: "Cairo (UTC+2)" },
  { value: "Europe/London", label: "London (UTC+0)" },
  { value: "America/New_York", label: "New York (UTC-5)" },
  { value: "Asia/Kolkata", label: "Kolkata (UTC+5:30)" },
];

function validateStep1(): boolean {
  if (!orgName.trim()) { error = "Organization name is required"; return false; }
  return true;
}

function validateStep2(): boolean {
  if (!adminFullName.trim()) { error = "Admin name is required"; return false; }
  if (!adminEmail.trim()) { error = "Admin email is required"; return false; }
  if (!adminPassword) { error = "Password is required"; return false; }
  if (adminPassword.length < 8) { error = "Password must be at least 8 characters"; return false; }
  if (adminPassword !== adminPasswordConfirm) { error = "Passwords do not match"; return false; }
  return true;
}

function goToStep2() {
  error = "";
  if (validateStep1()) step = 2;
}

async function bootstrap() {
  error = "";
  if (!validateStep2()) return;

  isSubmitting = true;
  try {
    const result = await api.bootstrapOrganization({
      orgName: orgName.trim(),
      orgNameAr: orgNameAr.trim() || undefined,
      orgType,
      timezone,
      locale,
      currency,
      adminFullName: adminFullName.trim(),
      adminEmail: adminEmail.trim(),
      adminPassword,
    });

    // Store session
    if (result.token) {
      localStorage.setItem("orbis_token", result.token);
      await authStore.restore();
    }

    // Navigate to org dashboard
    goto(`/${result.org_id}`, { replaceState: true });
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isSubmitting = false;
  }
}
</script>

<svelte:head>
	<title>Orbis — Create Organization</title>
</svelte:head>

<div class="setup-card">
	<div class="setup-header">
		<div class="step-indicator">
			<div class="step-dot" class:active={step >= 1}></div>
			<div class="step-line" class:active={step >= 2}></div>
			<div class="step-dot" class:active={step >= 2}></div>
		</div>
		<h1>{step === 1 ? "Create Your Organization" : "Create Admin Account"}</h1>
		<p class="subtitle">
			{step === 1
				? "Set up your institution's basic information"
				: "Create the administrator account"}
		</p>
	</div>

	{#if step === 1}
		<!-- Organization Info -->
		<div class="form-section">
			<div class="field-row">
				<div class="field-group">
					<label for="org-name">Organization Name *</label>
					<input id="org-name" type="text" bind:value={orgName} placeholder="Riyadh Academy" />
				</div>
				<div class="field-group">
					<label for="org-name-ar">الاسم بالعربية</label>
					<input id="org-name-ar" type="text" bind:value={orgNameAr} placeholder="أكاديمية الرياض" dir="rtl" />
				</div>
			</div>

			<div class="field-group">
				<label>Institution Type</label>
				<div class="type-selector">
					{#each orgTypes as t}
						<button
							class="type-chip"
							class:active={orgType === t.value}
							onclick={() => orgType = t.value}
						>
							<Building2 size={14} />
							{t.label}
						</button>
					{/each}
				</div>
			</div>

			<div class="field-row">
				<div class="field-group">
					<label for="currency">Currency</label>
					<select id="currency" bind:value={currency}>
						{#each currencies as c}
							<option value={c.code}>{c.label}</option>
						{/each}
					</select>
				</div>
				<div class="field-group">
					<label for="timezone">Timezone</label>
					<select id="timezone" bind:value={timezone}>
						{#each timezones as tz}
							<option value={tz.value}>{tz.label}</option>
						{/each}
					</select>
				</div>
			</div>
		</div>

		{#if error}
			<div class="error-banner">{error}</div>
		{/if}

		<button class="primary-btn" onclick={goToStep2}>
			Continue
			<ArrowRight size={16} />
		</button>

	{:else}
		<!-- Admin Account -->
		<div class="form-section">
			<div class="field-group">
				<label for="admin-name">Full Name *</label>
				<input id="admin-name" type="text" bind:value={adminFullName} placeholder="Ahmed Mohammed" />
			</div>

			<div class="field-group">
				<label for="admin-email">Email *</label>
				<input id="admin-email" type="email" bind:value={adminEmail} placeholder="admin@school.com" />
			</div>

			<div class="field-row">
				<div class="field-group">
					<label for="admin-password">Password *</label>
					<input id="admin-password" type="password" bind:value={adminPassword} placeholder="Minimum 8 characters" />
				</div>
				<div class="field-group">
					<label for="admin-password-confirm">Confirm Password *</label>
					<input id="admin-password-confirm" type="password" bind:value={adminPasswordConfirm} placeholder="Confirm password" />
				</div>
			</div>
		</div>

		{#if error}
			<div class="error-banner">{error}</div>
		{/if}

		<div class="btn-row">
			<button class="secondary-btn" onclick={() => { step = 1; error = ""; }}>
				<ArrowLeft size={16} />
				Back
			</button>
			<button class="primary-btn" onclick={bootstrap} disabled={isSubmitting}>
				{#if isSubmitting}
					<Loader2 size={16} class="spin" />
					Creating...
				{:else}
					Create Organization
					<ArrowRight size={16} />
				{/if}
			</button>
		</div>
	{/if}
</div>

<style>
	.setup-card {
		width: 100%;
		max-width: 560px;
		background: hsl(222 47% 11% / 0.9);
		border: 1px solid hsl(215 20% 22%);
		border-radius: 16px;
		padding: 2.5rem;
		backdrop-filter: blur(20px);
	}

	.setup-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.step-indicator {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0;
		margin-bottom: 1.25rem;
	}

	.step-dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		background: hsl(215 20% 25%);
		transition: all 0.3s;
	}

	.step-dot.active {
		background: hsl(210 100% 55%);
		box-shadow: 0 0 8px hsl(210 100% 55% / 0.4);
	}

	.step-line {
		width: 60px;
		height: 2px;
		background: hsl(215 20% 25%);
		transition: all 0.3s;
	}

	.step-line.active {
		background: hsl(210 100% 55%);
	}

	h1 {
		font-size: 1.35rem;
		font-weight: 600;
		color: hsl(210 40% 96%);
		margin: 0 0 0.5rem;
	}

	.subtitle {
		font-size: 0.85rem;
		color: hsl(215 20% 55%);
		margin: 0;
	}

	.form-section {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
		margin-bottom: 1.5rem;
	}

	.field-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.75rem;
	}

	.field-group label {
		display: block;
		font-size: 0.8rem;
		font-weight: 500;
		color: hsl(215 20% 65%);
		margin-bottom: 0.4rem;
	}

	.field-group input,
	.field-group select {
		width: 100%;
		padding: 0.6rem 0.8rem;
		background: hsl(222 47% 8%);
		border: 1px solid hsl(215 20% 22%);
		border-radius: 8px;
		color: hsl(210 40% 96%);
		font-size: 0.85rem;
		outline: none;
		transition: border-color 0.15s;
	}

	.field-group input:focus,
	.field-group select:focus {
		border-color: hsl(210 100% 50%);
	}

	.field-group select {
		appearance: none;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath d='M3 5l3 3 3-3' stroke='%237a8ba5' stroke-width='1.5' fill='none'/%3E%3C/svg%3E");
		background-repeat: no-repeat;
		background-position: right 0.75rem center;
		padding-right: 2rem;
	}

	.type-selector {
		display: flex;
		gap: 0.5rem;
	}

	.type-chip {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.5rem;
		border: 1px solid hsl(215 20% 22%);
		border-radius: 8px;
		background: transparent;
		color: hsl(215 20% 55%);
		font-size: 0.8rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.type-chip:hover {
		border-color: hsl(210 100% 50% / 0.4);
	}

	.type-chip.active {
		border-color: hsl(210 100% 50%);
		background: hsl(210 100% 50% / 0.1);
		color: hsl(210 100% 70%);
	}

	.error-banner {
		padding: 0.65rem 0.875rem;
		background: hsl(0 70% 40% / 0.15);
		border: 1px solid hsl(0 70% 40% / 0.3);
		border-radius: 8px;
		color: hsl(0 70% 70%);
		font-size: 0.8rem;
		margin-bottom: 1.25rem;
	}

	.btn-row {
		display: flex;
		gap: 0.75rem;
	}

	.primary-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.7rem 1.25rem;
		background: hsl(210 100% 50%);
		border: none;
		border-radius: 10px;
		color: white;
		font-size: 0.85rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.primary-btn:hover:not(:disabled) {
		background: hsl(210 100% 55%);
		transform: translateY(-1px);
		box-shadow: 0 4px 16px hsl(210 100% 50% / 0.3);
	}

	.primary-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.secondary-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.7rem 1rem;
		background: hsl(215 20% 18%);
		border: 1px solid hsl(215 20% 28%);
		border-radius: 10px;
		color: hsl(210 40% 80%);
		font-size: 0.85rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.secondary-btn:hover {
		background: hsl(215 20% 22%);
	}

	:global(.spin) {
		animation: spin 0.9s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}
</style>
