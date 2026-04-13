import { PAPER_PRESETS } from "../components/certificates/editor/constants"; // I will move PAPER_PRESETS to a shared place later if needed

export interface EditorElement {
  id: string;
  type: "text" | "image" | "shape" | "variable" | "frame" | "table";
  name: string;
  x: number;
  y: number;
  width: number;
  height: number;
  rotation: number;
  opacity: number;
  zIndex: number;
  visible: boolean;
  locked: boolean;
  content?: any;
  variableKey?: string;
  style: {
    color?: string;
    backgroundColor?: string;
    fontSize?: number;
    fontWeight?: string;
    fontFamily?: string;
    textAlign?: "left" | "center" | "right";
    lineHeight?: number;
    letterSpacing?: number;
    borderRadius?: number;
    borderWidth?: number;
    borderColor?: string;
    shadow?: string;
    direction?: "ltr" | "rtl";
  };
  interactions?: {
    trigger: "onClick" | "onHover";
    action: "openUrl" | "navigate" | "openModal";
    value: string;
  }[];
}

export interface EditorState {
  name: string;
  mode: "design" | "prototype";
  activePageId: string;
  elements: EditorElement[];
  selectedElementIds: string[];
  canvasConfig: {
    size: typeof PAPER_PRESETS[number]["id"];
    orientation: "portrait" | "landscape";
    backgroundColor: string;
    zoom: number;
    showGrid: boolean;
    gridSize: number;
    gridColumns: number;
    gridColor: string;
    showRulers: boolean;
    layoutConfig?: {
      rows: number;
      columns: number;
      gap: number;
    };
  };
  history: {
    past: EditorElement[][];
    future: EditorElement[][];
  };
}
