import { createCameraApp } from "../camera-settings";
import { Logo } from "./assets/logo";

const config = createCameraApp({
  protocol: "dahua",
  name: "Dahua Camera",
  nameAr: "كاميرا Dahua",
  shortDescription: "Connect Dahua cameras and view live feeds.",
  shortDescriptionAr: "اربط كاميرات Dahua واعرض بثها المباشر.",
  logo: Logo,
});

export default config;
