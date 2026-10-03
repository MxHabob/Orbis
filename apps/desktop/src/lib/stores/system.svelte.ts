// ── System & Readiness Store ──────────────────────────────────────────
// Svelte 5 Runes-based system and organization readiness state.

import { api } from "$lib/api/client";
import type {
  ConnectionConfig,
  ConnectionMode,
  ConnectionValidation,
  OrganizationReadiness,
  SetupItem,
  CompleteOrganizationOnboardingRequest,
  OnboardingResult,
  EntitlementResult,
  OrgLifecycleState,
  BootstrapRequest,
  BootstrapResult,
  BootstrapStatusResponse,
} from "$lib/api/types";
import { extractErrorMessage } from "$lib/api";

class SystemStore {
  // Connection state
  connectionConfig = $state<ConnectionConfig | null>(null);
  connectionState = $state<"unconfigured" | "connected" | "error" | "validating">("unconfigured");
  connectionMode = $state<ConnectionMode | null>(null);
  connectionError = $state<string | null>(null);

  // Organization readiness & onboarding
  orgReadiness = $state<OrganizationReadiness | null>(null);
  onboardingState = $state<import("$lib/api/types").OnboardingStateDto | null>(null);
  entitlement = $state<EntitlementResult | null>(null);
  isLoading = $state(false);

  // Derived state
  isConnected = $derived(this.connectionState === "connected");
  hasOrganization = $derived(this.orgReadiness?.has_organization ?? false);
  orgLifecycleState = $derived<OrgLifecycleState>(this.orgReadiness?.state ?? "empty");
  isOperational = $derived(this.orgReadiness?.state === "operational");
  
  // Real DAG onboarding tasks
  onboardingTasks = $derived(this.onboardingState?.tasks ?? []);
  pendingOnboardingTasks = $derived(
    this.onboardingState?.tasks.filter((t) => t.status !== "completed" && t.status !== "not_applicable") ?? []
  );
  completedOnboardingTasks = $derived(
    this.onboardingState?.tasks.filter((t) => t.status === "completed") ?? []
  );
  blockedOnboardingTasks = $derived(
    this.onboardingState?.tasks.filter((t) => t.status === "blocked") ?? []
  );
  onboardingProgressPercentage = $derived(this.onboardingState?.progress_percentage ?? 0);
  isInstitutionReady = $derived(this.onboardingState?.is_institution_ready ?? false);
  pendingSetupItems = $derived(
    this.orgReadiness?.pending_setup_items?.filter((i) => !i.is_completed) ?? []
  );
  completedSetupItems = $derived(
    this.orgReadiness?.pending_setup_items?.filter((i) => i.is_completed) ?? []
  );
  setupProgressPercentage = $derived.by(() => {
    const total = this.orgReadiness?.pending_setup_items?.length ?? 0;
    if (total === 0) return 100;
    const completed = this.completedSetupItems.length;
    return Math.round((completed / total) * 100);
  });

  async loadConnectionConfig(): Promise<ConnectionConfig | null> {
    try {
      this.connectionConfig = await api.connectionGetConfig();
      if (this.connectionConfig) {
        this.connectionMode = this.connectionConfig.mode;
      }
      return this.connectionConfig;
    } catch (e) {
      console.warn("Failed to load connection config:", e);
      return null;
    }
  }

  async validateConnection(): Promise<ConnectionValidation> {
    this.connectionState = "validating";
    this.connectionError = null;
    try {
      const res = await api.connectionValidate();
      if (res.is_valid) {
        this.connectionState = "connected";
      } else {
        this.connectionState = "error";
        this.connectionError = res.message;
      }
      return res;
    } catch (e) {
      this.connectionState = "error";
      this.connectionError = extractErrorMessage(e);
      return {
        is_valid: false,
        mode: this.connectionMode || "local",
        message: this.connectionError,
      };
    }
  }

  async loadReadiness(): Promise<OrganizationReadiness | null> {
    try {
      this.orgReadiness = await api.systemGetReadiness();
      return this.orgReadiness;
    } catch (e) {
      console.warn("Failed to load organization readiness:", e);
      return null;
    }
  }

  async loadEntitlement(): Promise<EntitlementResult | null> {
    try {
      this.entitlement = await api.systemGetEntitlement();
      return this.entitlement;
    } catch (e) {
      console.warn("Failed to load entitlement:", e);
      return null;
    }
  }

  async bootstrapGetStatus(): Promise<BootstrapStatusResponse | null> {
    try {
      return await api.bootstrapGetStatus();
    } catch (e) {
      console.warn("Failed to get bootstrap status:", e);
      return null;
    }
  }

  async bootstrapOrganization(req: BootstrapRequest): Promise<BootstrapResult> {
    this.isLoading = true;
    try {
      const res = await api.bootstrapOrganization(req);
      await this.loadReadiness();
      return res;
    } finally {
      this.isLoading = false;
    }
  }

  async loadOnboardingState(orgId?: string): Promise<import("$lib/api/types").OnboardingStateDto | null> {
    try {
      this.onboardingState = await api.onboardingGetState(orgId);
      return this.onboardingState;
    } catch (e) {
      console.warn("Failed to load onboarding state:", e);
      return null;
    }
  }

  async provisionBlueprintDefaults(
    req: import("$lib/api/types").ProvisionDefaultsReqDto,
    orgId?: string,
  ): Promise<import("$lib/api/types").ProvisionDefaultsResultDto> {
    this.isLoading = true;
    try {
      const res = await api.onboardingProvisionDefaults(req, orgId);
      this.onboardingState = res.state;
      await this.loadReadiness();
      return res;
    } finally {
      this.isLoading = false;
    }
  }

  reset() {
    this.connectionConfig = null;
    this.connectionState = "unconfigured";
    this.connectionMode = null;
    this.connectionError = null;
    this.orgReadiness = null;
    this.entitlement = null;
    this.isLoading = false;
  }
}

export const systemStore = new SystemStore();
