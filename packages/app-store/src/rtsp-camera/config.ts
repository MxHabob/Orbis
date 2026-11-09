import { createCameraApp } from "../camera-settings";
import { Logo } from "./assets/logo";

const config = createCameraApp({
  protocol: "rtsp",
  name: "RTSP Camera",
  nameAr: "كاميرا RTSP",
  shortDescription: "Connect RTSP-compatible cameras and view live feeds.",
  shortDescriptionAr: "اربط الكاميرات الداعمة لـ RTSP واعرض بثها المباشر.",
  logo: Logo,
});

export default config;
