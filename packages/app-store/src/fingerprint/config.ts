import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "fingerprint",
  name: "Fingerprint Scanner",
  name_ar: "ماسح البصمة",
  category: "hardware",
  active: true,
  beta: true,
  icon: "Fingerprint",
  short_description: "Connect ZKTeco or similar fingerprint scanners for biometric attendance tracking.",
  short_description_ar: "ربط أجهزة البصمة ZKTeco لتسجيل الحضور البيومتري.",
  description: `Automate attendance tracking with biometric fingerprint scanners.

**Automated Attendance**
Students and staff check in/out by fingerprint. Attendance records sync to Orbis in real-time.

**ZKTeco Support**
Compatible with ZKTeco U-series and other devices supporting the ZK protocol.`,
  features: ["Biometric attendance tracking", "Real-time sync to Orbis", "ZKTeco device support", "Multi-device support", "Offline buffering"],
  settings: [
    { id: "device_ip", label: "Device IP", type: "text", required: true, value: "", description: "IP address of the fingerprint device.", placeholder: "192.168.1.201" },
    { id: "device_port", label: "Device Port", type: "number", required: true, value: 4370, description: "Communication port.", validation: { min: 1, max: 65535 } },
    { id: "auto_sync", label: "Auto Sync Attendance", type: "switch", required: false, value: true, description: "Automatically sync attendance records to Orbis." },
  ],
  adapter_name: undefined,
  provider_trait: "IntegrationProvider",
};

export default config;
