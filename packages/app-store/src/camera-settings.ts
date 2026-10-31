import type { AppSetting, OrbisApp } from "./types";

export type CameraProtocol = "hikvision" | "dahua" | "onvif" | "rtsp";

interface CameraAppOptions {
  protocol: CameraProtocol;
  name: string;
  nameAr: string;
  shortDescription: string;
  shortDescriptionAr: string;
  logo?: OrbisApp["logo"];
}

export function createCameraApp({
  protocol,
  name,
  nameAr,
  shortDescription,
  shortDescriptionAr,
  logo,
}: CameraAppOptions): OrbisApp {
  const isRtsp = protocol === "rtsp";
  const protocolLabel = {
    hikvision: "Hikvision ISAPI",
    dahua: "Dahua CGI",
    onvif: "ONVIF",
    rtsp: "RTSP",
  }[protocol];

  const settings: AppSetting[] = [
    ...(isRtsp
      ? [
          {
            id: "rtsp_url",
            label: "RTSP Stream URL",
            label_ar: "عنوان بث RTSP",
            description: "Camera stream URL without embedded credentials.",
            description_ar: "عنوان بث الكاميرا دون تضمين بيانات الدخول.",
            type: "url" as const,
            required: true,
            value: "",
            placeholder: "rtsp://192.168.1.100:554/Streaming/Channels/101",
          },
        ]
      : [
          {
            id: "ip_address",
            label: "Camera IP Address or Hostname",
            label_ar: "عنوان IP أو اسم نطاق الكاميرا",
            description: `Network address of the ${name} camera or NVR.`,
            description_ar: `عنوان الشبكة لكاميرا ${name} أو جهاز NVR.`,
            type: "text" as const,
            required: true,
            value: "",
            placeholder: "192.168.1.100",
          },
          {
            id: "port",
            label: "Camera API Port",
            label_ar: "منفذ واجهة الكاميرا",
            description: "HTTP port for the camera API (default: 80).",
            description_ar: "منفذ HTTP لواجهة الكاميرا (الافتراضي: 80).",
            type: "number" as const,
            required: true,
            value: 80,
            validation: { min: 1, max: 65535 },
          },
        ]),
    {
      id: "username",
      label: "Username",
      label_ar: "اسم المستخدم",
      description: "Camera account used to verify the device connection.",
      description_ar: "حساب الكاميرا المستخدم للتحقق من الاتصال بالجهاز.",
      type: "text",
      required: true,
      value: "",
    },
    {
      id: "password",
      label: "Password",
      label_ar: "كلمة المرور",
      description: "Stored as an integration secret.",
      description_ar: "تُحفظ كسر تكامل.",
      type: "password",
      required: true,
      value: "",
    },
    {
      id: "whep_url",
      label: "MediaMTX HTTPS WHEP URL",
      label_ar: "عنوان HTTPS WHEP لخدمة MediaMTX",
      description:
        "HTTPS endpoint for this camera's MediaMTX path, ending in /whep.",
      description_ar:
        "عنوان HTTPS لمسار هذه الكاميرا في MediaMTX وينتهي بـ /whep.",
      type: "url",
      required: true,
      value: "",
      placeholder: "https://stream.example.com/campus-east/whep",
    },
    {
      id: "camera_name",
      label: "Camera Name",
      label_ar: "اسم الكاميرا",
      description: "Optional name shown in the live camera grid.",
      description_ar: "اسم اختياري يظهر في شبكة البث المباشر.",
      type: "text",
      required: false,
      value: "",
      placeholder: name,
    },
    {
      id: "location",
      label: "Location",
      label_ar: "الموقع",
      description: "Optional camera location shown in the live grid.",
      description_ar: "موقع اختياري للكاميرا يظهر في شبكة البث.",
      type: "text",
      required: false,
      value: "",
    },
  ];

  return {
    id: `${protocol}-camera`,
    name,
    name_ar: nameAr,
    category: "surveillance",
    active: true,
    icon: "Camera",
    logo,
    short_description: shortDescription,
    short_description_ar: shortDescriptionAr,
    description: `Connect a ${name} to Orbis. Orbis verifies its connection using ${protocolLabel}, then plays live video through a separately configured MediaMTX WebRTC/WHEP relay. Camera credentials remain server-side. Configure the relay's RTSP source separately.`,
    description_ar: `اربط ${name} بنظام Orbis. يتحقق Orbis من الاتصال باستخدام ${protocolLabel}، ثم يعرض الفيديو عبر وسيط MediaMTX WebRTC/WHEP مُعدّ بشكل منفصل. تبقى بيانات اعتماد الكاميرا على الخادم. اضبط مصدر RTSP في الوسيط بشكل منفصل.`,
    features: [
      `${protocolLabel} connection verification`,
      "MediaMTX WebRTC/WHEP live playback",
      "Server-side camera credentials",
    ],
    settings,
    adapter_name: `${protocol}_camera`,
    provider_trait: "IntegrationProvider",
    supports_live: true,
  };
}
