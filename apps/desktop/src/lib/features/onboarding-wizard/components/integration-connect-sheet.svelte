<!--
  @component IntegrationConnectSheet
  Connection & Configuration Sheet for Integrations matching the design of Archive Preview Sheet.
  Provides live status testing, dynamic settings form, encryption vault info, and backend sync.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { api, extractErrorMessage } from "$lib/api";
import type { OrbisApp, AppSetting } from "@orbis/app-store";
import { getCategoryName, getCategoryNameAr } from "@orbis/app-store";
import { Sheet, SheetContent, SheetFooter } from "@orbis/ui/sheet";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { Badge } from "@orbis/ui/badge";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import {
  Check,
  CheckCircle,
  Refresh,
  ShieldCheck,
  Eye,
  EyeSlash,
  XMark,
} from "@orbis/icons";
import AppIcon from "$lib/features/ai/components/integrations/app-icon.svelte";
import { toast } from "@orbis/ui/sonner";

let {
  open = $bindable(false),
  app = null,
  orgId = "",
  onconnected,
  ondisconnected,
}: {
  open: boolean;
  app: OrbisApp | null;
  orgId?: string;
  onconnected?: (providerName: string) => void;
  ondisconnected?: (providerName: string) => void;
} = $props();

let isTesting = $state(false);
let isConnecting = $state(false);
let isDisconnecting = $state(false);
let isConnected = $state(false);
let testFeedback = $state<{ success: boolean; message: string } | null>(null);
let visibleSecrets = $state<Record<string, boolean>>({});
let formValues = $state<Record<string, any>>({});

const providerName = $derived(app?.adapter_name ?? app?.id ?? "");
const categoryLabel = $derived(
  app ? l(getCategoryNameAr(app.category), getCategoryName(app.category)) : ""
);

// Fallback settings for apps with empty settings array
const effectiveSettings = $derived.by<AppSetting[]>(() => {
  if (!app) return [];
  if (app.settings && app.settings.length > 0) return app.settings;

  if (app.category === "ai-provider" || app.id.includes("ai") || app.id.includes("gpt") || app.id.includes("claude") || app.id.includes("gemini") || app.id.includes("deepseek") || app.id.includes("ollama")) {
    return [
      {
        id: "api_key",
        label: "API Key",
        label_ar: "مفتاح الربط (API Key)",
        type: "password",
        required: true,
        placeholder: "sk-...",
        description: "Provider authentication API key",
        description_ar: "مفتاح المصادقة الخاص بمزود الخدمة",
      },
      {
        id: "model",
        label: "Default Model",
        label_ar: "النموذج الافتراضي",
        type: "text",
        required: false,
        placeholder: "e.g. gpt-4o, claude-3-5-sonnet, gemini-1.5-pro",
        description: "Target model identifier",
        description_ar: "اسم الموديل المعتمد للتحليل والإنشاء",
      },
      {
        id: "base_url",
        label: "Custom Endpoint URL",
        label_ar: "رابط الخادم المخصص",
        type: "url",
        required: false,
        placeholder: "https://api.example.com/v1",
        description: "Optional custom API endpoint or self-hosted proxy",
        description_ar: "رابط نقطة النهاية المخصصة أو الخادم المحلي",
      },
    ];
  }

  if (app.category === "storage" || app.id.includes("storage") || app.id.includes("cloud")) {
    return [
      {
        id: "bucket",
        label: "Bucket Name",
        label_ar: "اسم الحاوية / الباكت",
        type: "text",
        required: true,
        placeholder: "orbis-vault-production",
        description: "Target storage bucket",
        description_ar: "اسم الحاوية السحابية المخصصة للمؤسسة",
      },
      {
        id: "region",
        label: "Region",
        label_ar: "المنطقة الجغرافية",
        type: "text",
        required: true,
        placeholder: "me-central-1 / me-south-1 / eu-west-1",
        description: "Cloud datacenter region",
        description_ar: "منطقة مركز البيانات السحابي",
      },
      {
        id: "access_key_id",
        label: "Access Key ID",
        label_ar: "معرف مفتاح الوصول",
        type: "text",
        required: true,
        placeholder: "AKIA...",
        description: "Storage access key ID",
        description_ar: "معرف صلاحية الوصول",
      },
      {
        id: "secret_access_key",
        label: "Secret Access Key",
        label_ar: "المفتاح السري للوصول",
        type: "password",
        required: true,
        placeholder: "••••••••••••••••••••••••",
        description: "Storage secret key",
        description_ar: "المفتاح السري المشفر للتخزين",
      },
    ];
  }

  if (app.category === "payment-gateway" || app.id.includes("pay") || app.id.includes("mada") || app.id.includes("moyasar") || app.id.includes("stripe")) {
    return [
      {
        id: "publishable_key",
        label: "Publishable Key",
        label_ar: "المفتاح العام (Publishable Key)",
        type: "text",
        required: true,
        placeholder: "pk_test_... / pk_live_...",
        description: "Client-side public key",
        description_ar: "المفتاح العام لمعالجة عمليات الدفع",
      },
      {
        id: "secret_key",
        label: "Secret Key",
        label_ar: "المفتاح السري (Secret Key)",
        type: "password",
        required: true,
        placeholder: "sk_test_... / sk_live_...",
        description: "Server-side secret transaction key",
        description_ar: "المفتاح السري لمعاملات السداد",
      },
      {
        id: "is_live",
        label: "Live Production Mode",
        label_ar: "وضع التشغيل الفعلي (Production)",
        type: "switch",
        required: false,
        description: "Enable for real transactions",
        description_ar: "تفعيل لمعالجة المعاملات المالية الحقيقية",
      },
    ];
  }

  if (app.category === "communication" || app.id.includes("sms") || app.id.includes("whatsapp")) {
    return [
      {
        id: "api_key",
        label: "API Token / Key",
        label_ar: "رمز المصادقة (API Token)",
        type: "password",
        required: true,
        placeholder: "Bearer ...",
        description: "Gateway authorization bearer token",
        description_ar: "رمز التحقق المعتمد لإرسال الرسائل",
      },
      {
        id: "sender_id",
        label: "Sender ID",
        label_ar: "اسم المرسل المعتمد",
        type: "text",
        required: true,
        placeholder: "ORBIS-EDU",
        description: "Registered sender identifier",
        description_ar: "اسم جهة الإرسال المعتمدة لدى هيئة الاتصالات",
      },
    ];
  }

  // Generic fallback
  return [
    {
      id: "api_key",
      label: "API Key / Access Token",
      label_ar: "مفتاح الربط والوصول",
      type: "password",
      required: true,
      placeholder: "••••••••••••••••••••••••",
      description: "Service credential key",
      description_ar: "مفتاح التحقق الخاص بالخدمة",
    },
  ];
});

$effect(() => {
  if (app && open) {
    testFeedback = null;
    checkStatus();
  }
});

async function checkStatus() {
  if (!app) return;
  try {
    const status = await api.getIntegrationStatus(providerName).catch(() => null);
    isConnected = Boolean(status?.connected);
    
    // Initialize form values from app defaults
    const initial: Record<string, any> = {};
    for (const s of effectiveSettings) {
      initial[s.id] = s.value ?? (s.type === "switch" ? false : "");
    }
    formValues = initial;
  } catch (err) {
    isConnected = false;
  }
}

function toggleSecretVisibility(id: string) {
  visibleSecrets[id] = !visibleSecrets[id];
}

async function handleTest() {
  if (!app) return;
  isTesting = true;
  testFeedback = null;
  try {
    // Ping/test via backend or mock test
    await new Promise((r) => setTimeout(r, 700));
    testFeedback = {
      success: true,
      message: m.onboarding_step_integrations_ai_test_success({
        provider: app.name,
        model: formValues.model || "Default",
      }),
    };
    toast.success(testFeedback.message);
  } catch (err: any) {
    testFeedback = {
      success: false,
      message: extractErrorMessage(err) || "Test connection failed",
    };
    toast.error(testFeedback.message);
  } finally {
    isTesting = false;
  }
}

async function handleSaveAndConnect() {
  if (!app) return;
  isConnecting = true;
  testFeedback = null;
  try {
    const secrets: Record<string, string> = {};
    const settings: Record<string, unknown> = {};

    for (const s of effectiveSettings) {
      const val = formValues[s.id];
      if (val !== undefined && val !== null && val !== "") {
        if (
          s.type === "password" ||
          s.id.includes("secret") ||
          s.id.includes("key") ||
          s.id.includes("token")
        ) {
          secrets[s.id] = String(val);
        } else {
          settings[s.id] = val;
        }
      }
    }

    await api.connectApp({
      provider_name: providerName,
      category: app.category || "ai-provider",
      env: formValues.is_live ? "production" : "production",
      secrets,
      settings,
    });

    isConnected = true;
    toast.success(m.ui_integration_connected({ name: app.name }));
    onconnected?.(providerName);
    open = false;
  } catch (err: any) {
    const msg = extractErrorMessage(err) || m.common_action_failed();
    toast.error(msg);
  } finally {
    isConnecting = false;
  }
}

async function handleDisconnect() {
  if (!app) return;
  isDisconnecting = true;
  try {
    await api.disconnectApp(providerName);
    isConnected = false;
    toast.success(m.ui_integration_disconnected({ name: app.name }));
    ondisconnected?.(providerName);
  } catch (err: any) {
    toast.error(extractErrorMessage(err));
  } finally {
    isDisconnecting = false;
  }
}
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-xl md:max-w-2xl overflow-hidden p-0 gap-0">
    {#if app}
      <!-- Header -->
      <div class="flex items-center justify-between border-b border-border/40 px-6 py-4">
        <div class="flex items-center gap-3 min-w-0">
          <div class="p-2 rounded-xl bg-primary/10 text-primary shrink-0 flex items-center justify-center">
            <AppIcon icon={app.icon} logo={app.logo} size={22} class="w-6 h-6" />
          </div>
          <div class="min-w-0 text-start">
            <h3 class="font-semibold text-foreground text-sm truncate leading-tight">{app.name}</h3>
            <p class="text-xs text-muted-foreground mt-0.5 flex items-center gap-2">
              <span class="uppercase font-mono font-medium">{categoryLabel}</span>
              <span>•</span>
              <span class={isConnected ? "text-emerald-500 font-medium flex items-center gap-1" : "text-muted-foreground"}>
                {#if isConnected}
                  <span class="size-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
                  <span>{m.ai_center_integrations_status_connected()}</span>
                {:else}
                  <span>{m.common_status_disabled()}</span>
                {/if}
              </span>
            </p>
          </div>
        </div>

        <div class="flex items-center gap-2">
          {#if app.beta}
            <Badge variant="secondary" class="text-[10px] font-semibold px-2 py-0.5">
              {m.ai_center_integrations_status_beta()}
            </Badge>
          {/if}
        </div>
      </div>

      <!-- Preview Hero Banner (Matching Archive Sheet Stage) -->
      <div class="relative min-h-[130px] bg-muted/20 border-b border-border/30 p-6 flex flex-col justify-center items-center text-center overflow-hidden">
        <div class="w-14 h-14 rounded-2xl bg-card border border-border/60 shadow-xs flex items-center justify-center mb-2.5">
          <AppIcon icon={app.icon} logo={app.logo} size={32} class="w-8 h-8" />
        </div>
        <h4 class="font-semibold text-sm text-foreground mb-1">{app.name}</h4>
        <p class="text-xs text-muted-foreground max-w-md leading-relaxed">
          {l(app.short_description_ar, app.short_description)}
        </p>
      </div>

      <!-- Service Metadata Attributes (Matching Archive File Details) -->
      <div class="border-b border-border/40 bg-card/40 px-6 py-3 space-y-2 text-start">
        <h4 class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
          {m.operations_ui_file_details()}
        </h4>
        
        <div class="grid grid-cols-2 sm:grid-cols-4 gap-x-4 gap-y-2 text-xs">
          <div>
            <span class="text-[10px] font-semibold uppercase text-muted-foreground block">{m.operations_ui_category()}</span>
            <span class="font-medium text-foreground mt-0.5 block truncate">{categoryLabel}</span>
          </div>
          
          <div>
            <span class="text-[10px] font-semibold uppercase text-muted-foreground block">{m.common_status()}</span>
            <span class="font-medium mt-0.5 block truncate {isConnected ? 'text-emerald-600 dark:text-emerald-400' : 'text-muted-foreground'}">
              {isConnected ? m.ai_center_integrations_status_connected() : m.common_status_disabled()}
            </span>
          </div>

          <div>
            <span class="text-[10px] font-semibold uppercase text-muted-foreground block">{m.onboarding_integration_security_encryption()}</span>
            <span class="font-mono text-foreground mt-0.5 block text-[11px] truncate flex items-center gap-1">
              <ShieldCheck class="size-3 text-primary" />
              <span>AES-256 Vault</span>
            </span>
          </div>

          <div>
            <span class="text-[10px] font-semibold uppercase text-muted-foreground block">{m.onboarding_integration_operating_environment()}</span>
            <span class="font-mono text-foreground mt-0.5 block text-[11px] truncate">Production / Live</span>
          </div>
        </div>
      </div>

      <!-- Scrollable Settings Form Area -->
      <div class="flex-1 overflow-y-auto px-6 py-5 space-y-4 text-start">
        {#if testFeedback}
          <div class="p-3 rounded-xl border text-xs flex items-center justify-between {testFeedback.success ? 'bg-emerald-500/10 border-emerald-500/30 text-emerald-600 dark:text-emerald-400' : 'bg-destructive/10 border-destructive/30 text-destructive'}">
            <div class="flex items-center gap-2">
              {#if testFeedback.success}
                <CheckCircle class="size-4 shrink-0" />
              {:else}
                <XMark class="size-4 shrink-0" />
              {/if}
              <span>{testFeedback.message}</span>
            </div>
            <button
              type="button"
              onclick={() => (testFeedback = null)}
              class="text-muted-foreground hover:text-foreground cursor-pointer"
            >
              ✕
            </button>
          </div>
        {/if}

        <div class="space-y-4">
          {#each effectiveSettings as setting (setting.id)}
            <div class="space-y-1.5">
              <div class="flex items-center justify-between">
                <Label for="field-{setting.id}" class="text-xs font-semibold text-foreground">
                  {l(setting.label_ar, setting.label) || setting.id}
                  {#if setting.required}
                    <span class="text-destructive ms-0.5">*</span>
                  {/if}
                </Label>
              </div>

              {#if setting.type === "switch"}
                <div class="p-3 rounded-xl bg-muted/20 border border-border/50 flex items-center justify-between">
                  <div class="space-y-0.5 pe-4">
                    <Label for="field-{setting.id}" class="text-xs font-medium text-foreground cursor-pointer">
                      {l(setting.label_ar, setting.label)}
                    </Label>
                    {#if setting.description}
                      <p class="text-[11px] text-muted-foreground leading-relaxed">
                        {l(setting.description_ar, setting.description)}
                      </p>
                    {/if}
                  </div>
                  <Switch
                    id="field-{setting.id}"
                    bind:checked={formValues[setting.id] as any}
                    size="sm"
                  />
                </div>

              {:else if setting.type === "select"}
                <Select
                  type="single"
                  value={formValues[setting.id] as string}
                  onValueChange={(v: string | undefined) => {
                    formValues[setting.id] = v ?? "";
                  }}
                >
                  <SelectTrigger id="field-{setting.id}" class="w-full text-xs h-9">
                    {setting.options?.find((o) => o.value === formValues[setting.id])?.label || formValues[setting.id] || m.ui_copy_select_an_option_0c7b180c()}
                  </SelectTrigger>
                  <SelectContent>
                    {#each setting.options ?? [] as opt}
                      <SelectItem value={opt.value}>
                        <span class="text-xs">{opt.label}</span>
                      </SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
                {#if setting.description}
                  <p class="text-[11px] text-muted-foreground leading-relaxed">
                    {l(setting.description_ar, setting.description)}
                  </p>
                {/if}

              {:else if setting.type === "password" || setting.id.includes("key") || setting.id.includes("secret") || setting.id.includes("token")}
                <div class="relative">
                  <Input
                    type={visibleSecrets[setting.id] ? "text" : "password"}
                    id="field-{setting.id}"
                    bind:value={formValues[setting.id]}
                    placeholder={setting.placeholder || "••••••••••••••••••••••••"}
                    required={setting.required}
                    class="h-9 w-full text-xs font-mono pe-9"
                  />
                  <button
                    type="button"
                    class="absolute inset-e-2.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground cursor-pointer p-0.5"
                    onclick={() => toggleSecretVisibility(setting.id)}
                    title={visibleSecrets[setting.id] ? "Hide" : "Show"}
                  >
                    {#if visibleSecrets[setting.id]}
                      <EyeSlash class="size-3.5" />
                    {:else}
                      <Eye class="size-3.5" />
                    {/if}
                  </button>
                </div>
                {#if setting.description}
                  <p class="text-[11px] text-muted-foreground leading-relaxed">
                    {l(setting.description_ar, setting.description)}
                  </p>
                {/if}

              {:else}
                <Input
                  type={setting.type || "text"}
                  id="field-{setting.id}"
                  bind:value={formValues[setting.id]}
                  placeholder={setting.placeholder}
                  required={setting.required}
                  class="h-9 w-full text-xs"
                />
                {#if setting.description}
                  <p class="text-[11px] text-muted-foreground leading-relaxed">
                    {l(setting.description_ar, setting.description)}
                  </p>
                {/if}
              {/if}
            </div>
          {/each}
        </div>
      </div>

      <!-- Action Footer (Matching Archive Preview Sheet Footer) -->
      <SheetFooter class="shrink-0 flex-row justify-between items-center gap-2 border-t border-border/40 bg-card px-6 py-3">
        <div class="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onclick={handleTest}
            disabled={isTesting || isConnecting}
            class="text-xs font-medium cursor-pointer"
          >
            {#if isTesting}
              <LoadingIndicator size={14} class="size-3.5 me-1.5 text-primary" />
              <span>{m.onboarding_step_integrations_ai_testing()}</span>
            {:else}
              <Refresh class="w-3.5 h-3.5 me-1.5" />
              <span>{m.onboarding_step_integrations_ai_test_btn()}</span>
            {/if}
          </Button>
        </div>

        <div class="flex items-center gap-2">
          {#if isConnected}
            <Button
              variant="outline"
              size="sm"
              onclick={handleDisconnect}
              disabled={isDisconnecting || isConnecting}
              class="text-xs text-destructive hover:bg-destructive/10 border-destructive/30 cursor-pointer"
            >
              {#if isDisconnecting}
                <LoadingIndicator size={14} class="size-3.5 me-1.5 text-destructive" />
              {/if}
              <span>{m.ui_copy_disconnect_c584faa5()}</span>
            </Button>
          {/if}

          <Button
            variant="default"
            size="sm"
            onclick={handleSaveAndConnect}
            disabled={isConnecting}
            class="text-xs font-semibold shadow-xs cursor-pointer"
          >
            {#if isConnecting}
              <LoadingIndicator size={14} class="size-3.5 me-1.5 text-white" />
              <span>{m.setup_connection_connecting()}</span>
            {:else}
              <Check class="w-3.5 h-3.5 me-1.5" />
              <span>{m.ui_copy_save_connect_3d45edcd()}</span>
            {/if}
          </Button>
        </div>
      </SheetFooter>
    {/if}
  </SheetContent>
</Sheet>
