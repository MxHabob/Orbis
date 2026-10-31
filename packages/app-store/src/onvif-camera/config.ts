import { createCameraApp } from "../camera-settings";
import { Logo } from "./assets/logo";

const config = createCameraApp({
  protocol: "onvif",
  name: "ONVIF Camera",
  nameAr: "كاميرا ONVIF",
  shortDescription: "Connect ONVIF-compatible cameras and view live feeds.",
  shortDescriptionAr: "اربط الكاميرات المتوافقة مع ONVIF واعرض بثها المباشر.",
  logo: Logo,
});

export default config;
