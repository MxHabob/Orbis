// ═══════════════════════════════════════════════════════════════
// @orbis/app-store — Main Entry Point
// Aggregates all app configs into a single `apps` array.
// Pattern: same as @midday/app-store index.ts
// ═══════════════════════════════════════════════════════════════

import type { OrbisApp, AppCategory } from "./types";
import { addConnectionSummary } from "./connection-guides";

// ── AI Providers ────────────────────────────────────────────────
import geminiAi from "./gemini-ai/config";
import openai from "./openai/config";
import anthropic from "./anthropic/config";
import deepseek from "./deepseek/config";
import groq from "./groq/config";
import mistral from "./mistral/config";
import ollama from "./ollama/config";
import openrouter from "./openrouter/config";

// ── App Imports ─────────────────────────────────────────────────
import authen from "./authen/config";
import orbisAi from "./orbis-ai/config";
import orbisMobile from "./orbis-mobile/config";
import hikvisionCamera from "./hikvision-camera/config";
import smtpEmail from "./smtp-email/config";
import whatsappBusiness from "./whatsapp-business/config";
import smsProvider from "./sms-provider/config";
import pushNotifications from "./push-notifications/config";
import googleWorkspace from "./google-workspace/config";
import microsoft365 from "./microsoft-365/config";
import activeDirectory from "./active-directory/config";
import fingerprint from "./fingerprint/config";
import rfid from "./rfid/config";
import barcodeScanner from "./barcode-scanner/config";
import printer from "./printer/config";
import mulverseCloud from "./mulverse-cloud/config";
import cloudStorage from "./cloud-storage/config";
import ocrService from "./ocr-service/config";
import solverService from "./solver-service/config";

// ── Payment Gateways ────────────────────────────────────────────
import moyasar from "./moyasar/config";
import mada from "./mada/config";
import tabby from "./tabby/config";
import tamara from "./tamara/config";
import hyperpay from "./hyperpay/config";
import stripe from "./stripe/config";
import sadad from "./sadad/config";
import jaib from "./jaib/config";
import alKuraimi from "./alkuraimi/config";
import yemenWallet from "./yemen-wallet/config";
import yemenMobileSms from "./yemen-mobile-sms/config";

// ── Exported App Collection ─────────────────────────────────────

const appDefinitions: OrbisApp[] = [
  // Native & Core Apps
  authen,
  orbisAi,
  orbisMobile,
  // AI Providers & LLMs
  geminiAi,
  openai,
  anthropic,
  deepseek,
  groq,
  mistral,
  ollama,
  openrouter,
  // Payment Gateways
  moyasar,
  mada,
  tabby,
  tamara,
  hyperpay,
  stripe,
  sadad,
  jaib,
  alKuraimi,
  yemenWallet,
  // Surveillance
  hikvisionCamera,
  // Communication
  smtpEmail,
  whatsappBusiness,
  smsProvider,
  yemenMobileSms,
  pushNotifications,
  // Identity
  googleWorkspace,
  microsoft365,
  activeDirectory,
  // Hardware
  fingerprint,
  rfid,
  barcodeScanner,
  printer,
  // Storage
  mulverseCloud,
  cloudStorage,
  // AI & Automation Services
  ocrService,
  solverService,
];

export const apps: OrbisApp[] = appDefinitions.map(addConnectionSummary);

// ── Helpers ─────────────────────────────────────────────────────

/** Get a single app by its unique ID */
export function getAppById(id: string): OrbisApp | undefined {
  return apps.find((app) => app.id === id);
}

/** Get all apps in a given category */
export function getAppsByCategory(category: AppCategory | "all"): OrbisApp[] {
  if (category === "all") return apps;
  return apps.filter((app) => app.category === category);
}

/** Get all unique app IDs */
export function getAllAppIds(): string[] {
  return apps.map((app) => app.id);
}

// ── Re-exports ──────────────────────────────────────────────────
export type { OrbisApp, AppCategory, AppSetting, AppStatus } from "./types";
export { categories, getCategoryName, getCategoryNameAr } from "./categories";
export type { CategoryDef } from "./categories";
