import { Logo } from "./assets/logo";
import type { OrbisApp } from "../types";

const config: OrbisApp = {
  logo: Logo,
  id: "ocr-service",
  name: "OCR Service",
  name_ar: "خدمة التعرف الضوئي",
  category: "ai-service",
  active: false,
  icon: "ScanText",
  short_description:
    "Extract text from scanned documents, certificates, and student records using OCR.",
  short_description_ar:
    "استخراج النص من المستندات الممسوحة والشهادات باستخدام OCR.",
  description: `Optical Character Recognition for digitizing paper-based school documents.

**Document Digitization**
Scan and extract text from student records, certificates, and official documents.

**Arabic OCR**
Full support for Arabic text recognition alongside English.`,
  features: [
    "Arabic & English OCR",
    "Document digitization",
    "Batch processing",
    "PDF text extraction",
    "Handwriting recognition",
  ],
  settings: [
    {
      id: "provider",
      label: "OCR Provider",
      type: "select",
      required: true,
      value: "tesseract",
      description: "OCR engine to use.",
      options: [
        { label: "Tesseract (Local)", value: "tesseract" },
        { label: "Google Vision API", value: "google-vision" },
        { label: "Azure Computer Vision", value: "azure-vision" },
      ],
    },
    {
      id: "api_key",
      label: "API Key",
      type: "password",
      required: false,
      value: "",
      description:
        "API key for cloud OCR providers (not needed for Tesseract).",
    },
  ],
};

export default config;
