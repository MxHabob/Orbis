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
  const rtspSetting: AppSetting = {
    id: "rtsp_url",
    label: "RTSP Stream URL",
    label_ar: "عنوان بث RTSP",
    description: "Camera stream URL without embedded credentials.",
    description_ar: "عنوان بث الكاميرا دون تضمين بيانات الدخول.",
    type: "url",
    required: true,
    value: "",
    placeholder: "rtsp://192.168.1.100:554/Streaming/Channels/101",
  };

  const settings: AppSetting[] = [
    ...(isRtsp
      ? [rtspSetting]
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
          rtspSetting,
        ]),
    {
      id: "use_authentication",
      label: "Use camera credentials",
      label_ar: "إضافة بيانات دخول الكاميرا",
      description: "Enable only if the camera requires authentication.",
      description_ar: "فعّل هذا الخيار فقط إذا كانت الكاميرا تتطلب بيانات دخول.",
      type: "switch",
      required: false,
      value: false,
    },
    {
      id: "username",
      label: "Username",
      label_ar: "اسم المستخدم",
      description: isRtsp
        ? "Optional. Credentials are sent only when entered. Leave empty for a public stream."
        : "Camera account used to verify the device connection.",
      description_ar: isRtsp
        ? "اختياري. تُرسل بيانات الدخول عند إدخالها فقط. اتركه فارغاً للبث العام."
        : "حساب الكاميرا المستخدم للتحقق من الاتصال بالجهاز.",
      type: "text",
      required: false,
      value: "",
    },
    {
      id: "password",
      label: "Password",
      label_ar: "كلمة المرور",
      description: "Stored as an integration secret.",
      description_ar: "تُحفظ كسر تكامل.",
      type: "password",
      required: false,
      value: "",
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
    description: `Connect a ${name} to Orbis. Orbis verifies its ${protocolLabel} connection and manages a local MediaMTX relay while the live camera view is open. Enter the camera's RTSP stream URL; credentials are kept out of the URL and are not sent to the browser.`,
    description_ar: `اربط ${name} بنظام Orbis. يتحقق Orbis من اتصال ${protocolLabel} ويدير وسيط MediaMTX محلياً أثناء فتح صفحة البث. أدخل عنوان RTSP للكاميرا؛ تُحفظ بيانات الدخول منفصلة ولا تُرسل إلى المتصفح.`,
    features: [
      `${protocolLabel} connection verification`,
      "Managed local WebRTC relay",
      "On-demand camera streaming without transcoding",
    ],
    settings,
    adapter_name: `${protocol}_camera`,
    provider_trait: "IntegrationProvider",
    supports_live: true,
  };
}
