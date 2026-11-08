import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "jaib",
  name: "JAIB",
  name_ar: "جيب",
  category: "payment-gateway",
  active: false,
  beta: false,
  icon: "Wallet",
  short_description:
    "A digital wallet offering electronic payment and money transfer services.",
  short_description_ar:
    "محفظة رقمية ذكية تقدم خدمات الدفع الإلكتروني وتحويل الأموال.",
  description:
    "JAIB is a digital wallet offering financial services, electronic payments, and money transfers. Visit [the official JAIB website](https://e-jaib.com/ar) for product information.",
  description_ar:
    "جيب محفظة رقمية ذكية تقدم خدمات مالية مبتكرة، تشمل الدفع الإلكتروني وتحويل الأموال. للمزيد من المعلومات، تفضل بزيارة [الموقع الرسمي لجيب](https://e-jaib.com/ar).",
  features: [
    "Digital wallet services",
    "Electronic payments",
    "Money transfers",
  ],
  features_ar: ["خدمات المحفظة الرقمية", "الدفع الإلكتروني", "تحويل الأموال"],
  settings: [],
};

export default config;
