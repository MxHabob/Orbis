// ═══════════════════════════════════════════════════════════════
// App Categories — used for the filter bar in Integrations page
// Mirrors midday's `categories` array pattern.
// ═══════════════════════════════════════════════════════════════

import type { AppCategory } from "./types";

export interface CategoryDef {
  id: AppCategory | "all";
  name: string;
  name_ar: string;
}

export const categories: CategoryDef[] = [
  { id: "all", name: "All", name_ar: "الكل" },
  { id: "ai-provider", name: "AI Providers", name_ar: "مزودي الذكاء الاصطناعي" },
  { id: "surveillance", name: "Surveillance", name_ar: "المراقبة" },
  { id: "communication", name: "Communication", name_ar: "التواصل" },
  { id: "identity", name: "Identity", name_ar: "الهوية" },
  { id: "hardware", name: "Hardware", name_ar: "الأجهزة" },
  { id: "storage", name: "Storage", name_ar: "التخزين" },
  { id: "ai-service", name: "AI Services", name_ar: "خدمات الذكاء الاصطناعي" },
  { id: "ai-vision", name: "AI & Computer Vision", name_ar: "الذكاء الاصطناعي و الرؤية الحاسوبية" },
  { id: "payment-gateway", name: "Payment Gateways", name_ar: "بوابات الدفع الإلكتروني" },
  { id: "automation", name: "Automation", name_ar: "الأتمتة" },
];

/** Get display name for a category */
export function getCategoryName(categoryId: string): string {
  const cat = categories.find((c) => c.id === categoryId);
  return cat?.name ?? categoryId;
}

/** Get Arabic display name for a category */
export function getCategoryNameAr(categoryId: string): string {
  const cat = categories.find((c) => c.id === categoryId);
  return cat?.name_ar ?? categoryId;
}
