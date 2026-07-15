import type { OrbisApp } from "../types";

const config: OrbisApp = {
  id: "rfid",
  name: "RFID Reader",
  name_ar: "قارئ RFID",
  category: "hardware",
  active: false,
  icon: "Nfc",
  short_description: "Track student attendance and access control using RFID cards and readers.",
  short_description_ar: "تتبع حضور الطلاب والتحكم في الوصول باستخدام بطاقات RFID.",
  description: `Use RFID cards for contactless attendance tracking and campus access control.

**Card-Based Attendance**
Students tap their RFID cards at readers for instant check-in/check-out.

**Access Control**
Restrict access to specific areas (labs, libraries) based on student enrollment.`,
  features: ["Contactless attendance", "Campus access control", "Multiple reader support", "Card management", "Lost card handling"],
  settings: [
    { id: "reader_ip", label: "Reader IP", type: "text", required: true, value: "", description: "IP address of the RFID reader.", placeholder: "192.168.1.202" },
    { id: "reader_port", label: "Reader Port", type: "number", required: true, value: 8000, description: "Communication port.", validation: { min: 1, max: 65535 } },
  ],
  provider_trait: "IntegrationProvider",
};

export default config;
