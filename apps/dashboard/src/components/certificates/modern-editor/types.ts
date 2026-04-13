export type ElementType = "text" | "image" | "shape" | "variable" | "frame";

export interface EditorElement {
  id: string;
  name: string;
  type: ElementType;
  x: number;
  y: number;
  width: number;
  height: number;
  rotation: number;
  opacity: number;
  zIndex: number;
  visible: boolean;
  locked: boolean;
  content?: string;
  variableKey?: string;
  style: {
    color?: string;
    fontSize?: number;
    fontWeight?: string;
    fontFamily?: string;
    textAlign?: "left" | "center" | "right";
    backgroundColor?: string;
    borderRadius?: number;
    borderWidth?: number;
    borderColor?: string;
    padding?: number;
    lineHeight?: number;
    letterSpacing?: number;
    shadow?: string;
  };
  interactions: {
    trigger: "onClick" | "onHover";
    action: "navigate" | "openUrl" | "toast";
    value: string;
  }[];
}

export interface PaperPreset {
    id: string;
    name: string;
    width: number;
    height: number;
    category: "ISO A" | "US" | "Digital" | "Business";
}

export const PAPER_PRESETS: PaperPreset[] = [
    // ISO A
    { id: "a4", name: "A4", width: 794, height: 1123, category: "ISO A" },
    { id: "a5", name: "A5", width: 559, height: 794, category: "ISO A" },
    { id: "a3", name: "A3", width: 1123, height: 1587, category: "ISO A" },
    { id: "a6", name: "A6", width: 397, height: 559, category: "ISO A" },
    
    // US Styles
    { id: "letter", name: "Letter", width: 816, height: 1056, category: "US" },
    { id: "legal", name: "Legal", width: 816, height: 1344, category: "US" },
    { id: "tabloid", name: "Tabloid", width: 1056, height: 1632, category: "US" },

    // Digital / Presentation
    { id: "fhd", name: "Desktop (1920x1080)", width: 1920, height: 1080, category: "Digital" },
    { id: "insta", name: "Instagram Square", width: 1080, height: 1080, category: "Digital" },
    { id: "presentation", name: "Slide 16:9", width: 1920, height: 1080, category: "Digital" },

    // Business
    { id: "biz_us", name: "US Business Card", width: 336, height: 192, category: "Business" },
    { id: "biz_euro", name: "Euro Business Card", width: 321, height: 208, category: "Business" },
];

export type Orientation = "portrait" | "landscape";

export interface EditorState {
  mode: "design" | "prototype";
  activePageId: string;
  elements: EditorElement[];
  selectedElementIds: string[];
  canvasConfig: {
    size: string; // Preset ID or 'custom'
    orientation: Orientation;
    customWidth?: number;
    customHeight?: number;
    backgroundImage?: string;
    backgroundColor: string;
    zoom: number;
    showGrid: boolean;
    gridSize: number;
    gridColumns: number;
    gridColor: string;
    showRulers: boolean;
  };
  history: {
    past: EditorElement[][];
    future: EditorElement[][];
  };
}
