<!--
@component SettingsPage
AI Center Settings — Providers, Models, Autopilot config.
Wired to SDK: commands.aiCenter.testProviderConnection / saveProviderConfig / saveAutopilotConfig
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import {
  Brain,
  Sparkles,
  Zap,
  MessageSquare,
  Save,
  RefreshCw,
  CheckCircle,
  AlertTriangle,
} from "lucide-svelte";
import { Button } from "@orbis/ui-svelte/button";

let { data }: { data: { orgId: string } } = $props();

// ── Provider State ──────────────────────────────────────────
let providerSettings = $state({
  default_provider: "ollama" as "ollama" | "gemini",
  ollama_url: "http://localhost:11434",
  ollama_status: "unknown" as "connected" | "disconnected" | "unknown" | "testing",
  gemini_api_key: "",
  gemini_status: "unknown" as "connected" | "disconnected" | "unknown" | "testing",
});

// ── Model State ─────────────────────────────────────────────
let modelSettings = $state({
  default_model: "llama3.2",
  temperature: 0.7,
  max_tokens: 2048,
  top_p: 0.9,
});

// ── Autopilot State ─────────────────────────────────────────
let autopilotSettings = $state({
  enabled: false,
  execution_mode: "manual" as "automatic" | "suggested" | "manual",
  auto_open_enrollment: false,
  auto_close_enrollment: false,
  auto_record_attendance: false,
  auto_fee_assignment: false,
  auto_grade_entry_reminder: false,
  auto_period_lock: false,
  auto_promotion: false,
  auto_exam_scheduling: false,
  auto_report_generation: false,
  grace_days: 3,
  notify_before_deadline_hours: 24,
});

let saving = $state(false);

// ── Load Config ─────────────────────────────────────────────
onMount(async () => {
  try {
    const config = await commands.aiCenter.getAutopilotConfig(data.orgId);
    if (config) Object.assign(autopilotSettings, config);
  } catch (e) {
    console.warn("[AI Center] getAutopilotConfig not available:", e);
  }
});

// ── Actions ─────────────────────────────────────────────────
async function testOllama() {
  providerSettings.ollama_status = "testing";
  try {
    await commands.aiCenter.testProviderConnection("ollama", { base_url: providerSettings.ollama_url });
    providerSettings.ollama_status = "connected";
  } catch {
    providerSettings.ollama_status = "disconnected";
  }
}

async function testGemini() {
  providerSettings.gemini_status = "testing";
  try {
    await commands.aiCenter.testProviderConnection("gemini", { api_key: providerSettings.gemini_api_key });
    providerSettings.gemini_status = "connected";
  } catch {
    providerSettings.gemini_status = "disconnected";
  }
}

async function saveProviders() {
  saving = true;
  try {
    await commands.aiCenter.saveProviderConfig("ollama", { base_url: providerSettings.ollama_url });
    if (providerSettings.gemini_api_key) {
      await commands.aiCenter.saveProviderConfig("gemini", { api_key: providerSettings.gemini_api_key });
    }
  } catch (e) { console.warn(e); }
  saving = false;
}

async function saveModels() {
  saving = true;
  try {
    await commands.aiCenter.saveProviderConfig("model_config", modelSettings);
  } catch (e) { console.warn(e); }
  saving = false;
}

async function saveAutopilot() {
  saving = true;
  try {
    await commands.aiCenter.saveAutopilotConfig(data.orgId, autopilotSettings);
  } catch (e) { console.warn(e); }
  saving = false;
}

const autopilotToggles = [
  { id: "auto_open_enrollment", label: "Auto Open Enrollment", desc: "Automatically open enrollment when academic period starts" },
  { id: "auto_close_enrollment", label: "Auto Close Enrollment", desc: "Close enrollment at deadline" },
  { id: "auto_record_attendance", label: "Auto Record Attendance", desc: "Auto-capture from fingerprint/RFID devices" },
  { id: "auto_fee_assignment", label: "Auto Fee Assignment", desc: "Assign fees to new enrollments automatically" },
  { id: "auto_grade_entry_reminder", label: "Grade Entry Reminders", desc: "Notify teachers before grade submission deadlines" },
  { id: "auto_period_lock", label: "Auto Period Lock", desc: "Lock academic periods at deadline (Critical)" },
  { id: "auto_promotion", label: "Auto Promotion", desc: "Run student promotion at year end (Critical)" },
  { id: "auto_exam_scheduling", label: "Auto Exam Scheduling", desc: "Generate exam schedules automatically" },
  { id: "auto_report_generation", label: "Auto Reports", desc: "Generate reports at period close" },
] as const;

function statusIcon(s: string) {
  if (s === "connected") return { icon: CheckCircle, cls: "text-emerald-500", label: "Connected" };
  if (s === "disconnected") return { icon: AlertTriangle, cls: "text-red-500", label: "Disconnected" };
  if (s === "testing") return { icon: RefreshCw, cls: "text-muted-foreground animate-spin", label: "Testing..." };
  return { icon: null, cls: "", label: "Not tested" };
}

const ollamaStatusInfo = $derived(statusIcon(providerSettings.ollama_status));
const geminiStatusInfo = $derived(statusIcon(providerSettings.gemini_status));
</script>

<div class="h-full overflow-y-auto">
  <div class="px-8 py-6 max-w-3xl space-y-6">
    <div>
      <h1 class="text-lg font-semibold text-foreground mb-1">Settings</h1>
      <p class="text-[11px] text-muted-foreground">Configure AI providers, models, and autopilot behavior.</p>
    </div>

    <!-- ═══ Providers ═══ -->
    <section class="border border-border/60 rounded-lg overflow-hidden">
      <div class="px-5 py-3 bg-muted/30 border-b border-border/40 flex items-center gap-2">
        <Brain size={15} />
        <h2 class="text-sm font-medium text-foreground">AI Providers</h2>
      </div>
      <div class="p-5 space-y-4">
        <div class="space-y-1.5">
          <label class="text-xs font-medium text-foreground">Default Provider</label>
          <select bind:value={providerSettings.default_provider}
                  class="w-full text-xs border border-border/60 rounded-md px-3 py-2 bg-background">
            <option value="ollama">Ollama (Local)</option>
            <option value="gemini">Google Gemini (Cloud)</option>
          </select>
        </div>

        <!-- Ollama -->
        <div class="border border-border/40 rounded-lg p-4 space-y-3">
          <div class="flex items-center justify-between">
            <span class="text-xs font-medium text-foreground flex items-center gap-1.5"><Brain size={13} /> Ollama</span>
            {#if ollamaStatusInfo.icon}
              <span class="flex items-center gap-1 text-[10px] {ollamaStatusInfo.cls}"><svelte:component this={ollamaStatusInfo.icon} size={10} /> {ollamaStatusInfo.label}</span>
            {:else}
              <span class="text-[10px] text-muted-foreground">{ollamaStatusInfo.label}</span>
            {/if}
          </div>
          <input bind:value={providerSettings.ollama_url} type="url" placeholder="http://localhost:11434"
                 class="w-full text-xs border border-border/60 rounded-md px-3 py-2 bg-background" />
          <Button size="sm" variant="outline" onclick={testOllama} disabled={providerSettings.ollama_status === "testing"}>
            <RefreshCw size={12} class="mr-1.5" /> Test Connection
          </Button>
        </div>

        <!-- Gemini -->
        <div class="border border-border/40 rounded-lg p-4 space-y-3">
          <div class="flex items-center justify-between">
            <span class="text-xs font-medium text-foreground flex items-center gap-1.5"><Sparkles size={13} /> Google Gemini</span>
            {#if geminiStatusInfo.icon}
              <span class="flex items-center gap-1 text-[10px] {geminiStatusInfo.cls}"><svelte:component this={geminiStatusInfo.icon} size={10} /> {geminiStatusInfo.label}</span>
            {:else}
              <span class="text-[10px] text-muted-foreground">{geminiStatusInfo.label}</span>
            {/if}
          </div>
          <input bind:value={providerSettings.gemini_api_key} type="password" placeholder="AIza..."
                 class="w-full text-xs border border-border/60 rounded-md px-3 py-2 bg-background" />
          <Button size="sm" variant="outline" onclick={testGemini} disabled={providerSettings.gemini_status === "testing"}>
            <RefreshCw size={12} class="mr-1.5" /> Validate Key
          </Button>
        </div>

        <Button size="sm" onclick={saveProviders} disabled={saving}>
          <Save size={12} class="mr-1.5" /> Save Provider Settings
        </Button>
      </div>
    </section>

    <!-- ═══ Model Config ═══ -->
    <section class="border border-border/60 rounded-lg overflow-hidden">
      <div class="px-5 py-3 bg-muted/30 border-b border-border/40 flex items-center gap-2">
        <MessageSquare size={15} />
        <h2 class="text-sm font-medium text-foreground">Model Configuration</h2>
      </div>
      <div class="p-5 space-y-4">
        <div class="grid grid-cols-2 gap-4">
          <div class="space-y-1.5">
            <label class="text-xs font-medium text-foreground">Default Model</label>
            <select bind:value={modelSettings.default_model}
                    class="w-full text-xs border border-border/60 rounded-md px-3 py-2 bg-background">
              <optgroup label="Ollama">
                <option value="llama3.2">Llama 3.2</option>
                <option value="mistral">Mistral 7B</option>
                <option value="phi3:mini">Phi-3 Mini</option>
              </optgroup>
              <optgroup label="Gemini">
                <option value="gemini-2.0-flash">Gemini 2.0 Flash</option>
                <option value="gemini-2.0-pro">Gemini 2.0 Pro</option>
              </optgroup>
            </select>
          </div>
          <div class="space-y-1.5">
            <label class="text-xs font-medium text-foreground">Temperature ({modelSettings.temperature})</label>
            <input type="range" bind:value={modelSettings.temperature} min="0" max="2" step="0.1" class="w-full accent-primary" />
          </div>
          <div class="space-y-1.5">
            <label class="text-xs font-medium text-foreground">Max Tokens</label>
            <input type="number" bind:value={modelSettings.max_tokens} min="256" max="8192" step="256"
                   class="w-full text-xs border border-border/60 rounded-md px-3 py-2 bg-background" />
          </div>
          <div class="space-y-1.5">
            <label class="text-xs font-medium text-foreground">Top P ({modelSettings.top_p})</label>
            <input type="range" bind:value={modelSettings.top_p} min="0" max="1" step="0.05" class="w-full accent-primary" />
          </div>
        </div>
        <Button size="sm" onclick={saveModels} disabled={saving}>
          <Save size={12} class="mr-1.5" /> Save Model Settings
        </Button>
      </div>
    </section>

    <!-- ═══ Autopilot ═══ -->
    <section class="border border-border/60 rounded-lg overflow-hidden">
      <div class="px-5 py-3 bg-muted/30 border-b border-border/40 flex items-center gap-2">
        <Zap size={15} />
        <h2 class="text-sm font-medium text-foreground">Autopilot</h2>
      </div>
      <div class="p-5 space-y-4">
        <div class="flex items-center justify-between pb-3 border-b border-border/40">
          <label class="flex items-center gap-2 cursor-pointer">
            <input type="checkbox" bind:checked={autopilotSettings.enabled} class="rounded" />
            <span class="text-xs font-medium text-foreground">Enable Autopilot</span>
          </label>
          <select bind:value={autopilotSettings.execution_mode} disabled={!autopilotSettings.enabled}
                  class="text-xs border border-border/60 rounded-md px-2.5 py-1.5 bg-background disabled:opacity-50">
            <option value="automatic">Automatic</option>
            <option value="suggested">Suggested</option>
            <option value="manual">Manual</option>
          </select>
        </div>

        <div class="space-y-2.5" class:opacity-50={!autopilotSettings.enabled}>
          {#each autopilotToggles as toggle (toggle.id)}
            <label class="flex items-start gap-2.5 cursor-pointer">
              <input type="checkbox"
                     bind:checked={autopilotSettings[toggle.id]}
                     disabled={!autopilotSettings.enabled} class="rounded mt-0.5" />
              <div>
                <span class="text-xs font-medium text-foreground block">{toggle.label}</span>
                <span class="text-[10px] text-muted-foreground">{toggle.desc}</span>
              </div>
            </label>
          {/each}
        </div>

        <div class="grid grid-cols-2 gap-4 pt-3 border-t border-border/40">
          <div class="space-y-1.5">
            <label class="text-xs font-medium text-foreground">Grace Days</label>
            <input type="number" bind:value={autopilotSettings.grace_days} min="0" max="30"
                   disabled={!autopilotSettings.enabled}
                   class="w-full text-xs border border-border/60 rounded-md px-3 py-2 bg-background disabled:opacity-50" />
          </div>
          <div class="space-y-1.5">
            <label class="text-xs font-medium text-foreground">Notify Before (hours)</label>
            <input type="number" bind:value={autopilotSettings.notify_before_deadline_hours} min="1" max="168"
                   disabled={!autopilotSettings.enabled}
                   class="w-full text-xs border border-border/60 rounded-md px-3 py-2 bg-background disabled:opacity-50" />
          </div>
        </div>

        <Button size="sm" onclick={saveAutopilot} disabled={saving}>
          <Save size={12} class="mr-1.5" /> Save Autopilot Settings
        </Button>
      </div>
    </section>
  </div>
</div>
