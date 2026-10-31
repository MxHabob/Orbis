import { createCameraApp } from "../camera-settings";
import { Logo } from "./assets/logo";

const config = createCameraApp({
  protocol: "hikvision",
  name: "Hikvision Camera",
  nameAr: "كاميرا Hikvision",
  shortDescription: "Connect Hikvision cameras and view live feeds.",
  shortDescriptionAr: "اربط كاميرات Hikvision واعرض بثها المباشر.",
  logo: Logo,
});

export default config;
