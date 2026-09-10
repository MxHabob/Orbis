import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "barcode-scanner",
  name: "Barcode Scanner",
  name_ar: "ماسح الباركود",
  category: "hardware",
  active: false,
  icon: "ScanBarcode",
  short_description:
    "Use barcode and QR code scanners for student ID cards, library books, and inventory.",
  short_description_ar: "استخدام ماسحات الباركود وQR لبطاقات الطلاب والمكتبة.",
  description: `Integrate barcode and QR code scanners for various school operations.

**Student ID Cards**
Scan student ID barcodes for quick attendance and identification.

**Library Management**
Track book checkouts and returns with barcode scanning.`,
  features: [
    "Student ID scanning",
    "Library book tracking",
    "QR code support",
    "USB & wireless scanners",
    "Batch scanning",
  ],
  settings: [
    {
      id: "scanner_mode",
      label: "Scanner Mode",
      type: "select",
      required: true,
      value: "usb",
      description: "How the scanner connects.",
      options: [
        { label: "USB (HID)", value: "usb" },
        { label: "Network", value: "network" },
        { label: "Bluetooth", value: "bluetooth" },
      ],
    },
  ],
  provider_trait: "IntegrationProvider",
};

export default config;
