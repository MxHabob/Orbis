// ═══════════════════════════════════════════════════════════════
// @orbis/app-store — Type Definitions
// Inspired by @midday/app-store UnifiedApp pattern.
// Each integration is a self-contained "app" with metadata,
// settings schema, and connection lifecycle.
// ═══════════════════════════════════════════════════════════════

/** Application categories — maps to adapter trait groups in api-rust */
export type AppCategory =
  | "ai-provider" // LlmProvider trait (Ollama, Gemini)
  | "surveillance" // IntegrationProvider trait — camera category
  | "communication" // NotificationProvider trait (email, sms, push, whatsapp)
  | "identity" // Directory services (AD, LDAP, Google, M365)
  | "hardware" // Physical devices (fingerprint, RFID, barcode, printer)
  | "storage" // StorageProvider trait (local, S3, GCS)
  | "ai-service" // AI capabilities (OCR, STT, TTS)
  | "ai-vision" // AI and Computer Vision (Authen)
  | "payment-gateway" // Payment Gateways (Mada, Moyasar, Tabby, Tamara, etc.)
  | "automation"; // Webhooks, REST APIs, workflow engines

/** Connection status — mirrors AdapterStatus in infra-integration */
export type AppStatus = "connected" | "disconnected" | "error" | "disabled";

/** Individual setting field — used to auto-generate the connect form */
export interface AppSetting {
  /** Unique identifier for the setting */
  id: string;
  /** Display label (English) */
  label: string;
  /** Display label (Arabic) */
  label_ar?: string;
  /** Help text explaining the setting */
  description: string;
  /** Help text in Arabic */
  description_ar?: string;
  /** Input type — determines the rendered form control */
  type: "text" | "password" | "number" | "switch" | "select" | "url";
  /** Whether this setting must be filled to connect */
  required: boolean;
  /** Default value */
  value: unknown;
  /** Placeholder text for text inputs */
  placeholder?: string;
  /** Options for select-type settings */
  options?: Array<{ label: string; value: string }>;
  /** Validation constraints */
  validation?: {
    pattern?: string;
    min?: number;
    max?: number;
  };
}

/** Unified app/integration definition */
export interface OrbisApp {
  // ── Identity ──────────────────────────────────────────────────
  /** Unique slug identifier (e.g. "hikvision-camera") */
  id: string;
  /** Display name (English) */
  name: string;
  /** Display name (Arabic) */
  name_ar?: string;
  /** Category for grouping and filtering */
  category: AppCategory;

  // ── State ─────────────────────────────────────────────────────
  /** Whether this app is available for installation */
  active: boolean;
  /** Whether this app is in beta */
  beta?: boolean;

  // ── Presentation ──────────────────────────────────────────────
  /** Lucide icon name (e.g. "Camera", "Brain", "Sparkles") */
  icon: string;
  /** Custom logo component */
  logo?: any;
  /** One-line description (English) */
  short_description: string;
  /** One-line description (Arabic) */
  short_description_ar?: string;
  /** Full Markdown description (English) */
  description: string;
  /** Full Markdown description (Arabic) */
  description_ar?: string;
  /** Key features list */
  features: string[];
  /** Key features list (Arabic) */
  features_ar?: string[];

  /** App gallery/screenshot images */
  images?: string[];

  // ── Configuration ─────────────────────────────────────────────
  /** Settings schema — auto-generates the connect form */
  settings: AppSetting[];

  // ── Backend mapping ───────────────────────────────────────────
  /**
   * Name of the Rust adapter in `infra-integration/src/adapters/`.
   * undefined = adapter not yet implemented.
   */
  adapter_name?: string;
  /**
   * Which provider trait this app implements:
   * LlmProvider, IntegrationProvider, NotificationProvider, StorageProvider
   */
  provider_trait?: string;

  // ── Capabilities ──────────────────────────────────────────────
  /** Whether this app supports live video streaming */
  supports_live?: boolean;
  /** Whether connection requires OAuth flow */
  requires_oauth?: boolean;

  // ── Runtime (populated by frontend from backend data) ─────────
  /** Current connection status — filled at runtime, not in config */
  status?: AppStatus;
}
