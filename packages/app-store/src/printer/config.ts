import type { OrbisApp } from "../types";

const config: OrbisApp = {
  id: "printer",
  name: "Network Printer",
  name_ar: "طابعة الشبكة",
  category: "hardware",
  active: false,
  icon: "Printer",
  short_description: "Print report cards, certificates, and ID cards directly from Orbis.",
  short_description_ar: "طباعة بطاقات التقارير والشهادات وبطاقات الهوية مباشرة من Orbis.",
  description: `Connect network printers to print school documents directly from Orbis.

**Report Cards**
Print student report cards with grades, attendance, and teacher comments.

**Certificates**
Generate and print certificates, diplomas, and awards.

**ID Cards**
Print student and staff ID cards with photos and barcodes.`,
  features: ["Report card printing", "Certificate generation", "ID card printing", "Bulk print jobs", "PDF export"],
  settings: [
    { id: "printer_ip", label: "Printer IP", type: "text", required: true, value: "", description: "Network printer IP address.", placeholder: "192.168.1.100" },
    { id: "printer_name", label: "Printer Name", type: "text", required: false, value: "", description: "Display name for the printer.", placeholder: "Office Printer" },
  ],
};

export default config;
