import { create } from "zustand";
import { type EditorElement, type EditorState } from "../types";

interface EditorActions {
  // Mode Actions
  setMode: (mode: "design" | "prototype") => void;
  
  // Element Actions
  addElement: (element: Partial<EditorElement>) => void;
  updateElement: (id: string, updates: Partial<EditorElement>) => void;
  updateSelectedElements: (updates: Partial<EditorElement>) => void;
  deleteElements: (ids: string[]) => void;
  duplicateElements: (ids: string[]) => void;
  
  // Selection
  setSelectedElements: (ids: string[]) => void;
  
  // Layer Ordering
  bringToFront: (id: string) => void;
  sendToBack: (id: string) => void;
  bringForward: (id: string) => void;
  sendBackward: (id: string) => void;
  reorderElements: (startIndex: number, endIndex: number) => void;
  
  // Canvas Actions
  setCanvasConfig: (config: Partial<EditorState["canvasConfig"]>) => void;
  
  // History
  undo: () => void;
  redo: () => void;
  saveToHistory: () => void;
  
  // Batch Load
  setElements: (elements: EditorElement[]) => void;
}

export const useEditorStore = create<EditorState & EditorActions>((set, get) => ({
  mode: "design",
  activePageId: "page-1",
  elements: [],
  selectedElementIds: [],
  canvasConfig: {
    size: "a4",
    orientation: "portrait",
    backgroundColor: "#ffffff",
    zoom: 1,
    showGrid: true,
    gridSize: 20,
    gridColumns: 12,
    gridColor: "rgba(0,0,0,0.05)",
    showRulers: true,
  },
  history: {
    past: [],
    future: [],
  },

  setMode: (mode) => set({ mode }),
  setElements: (elements) => set({ elements }),

  saveToHistory: () => {
    const { elements, history } = get();
    set({
      history: {
        past: [...history.past, elements].slice(-50),
        future: [],
      }
    });
  },

  addElement: (element) => {
    get().saveToHistory();
    const newElement: EditorElement = {
      id: `el-${crypto.randomUUID()}`,
      name: element.name || (element.type === 'variable' ? `Variable: ${element.variableKey}` : `New ${element.type || 'text'}`),
      type: "text",
      x: 100,
      y: 100,
      width: 400,
      height: 80,
      rotation: 0,
      opacity: 1,
      zIndex: get().elements.length,
      visible: true,
      locked: false,
      interactions: [], // Best Practice 2026: Proto-ready by default
      ...element,
      style: {
        fontSize: 32,
        color: "#000000",
        fontFamily: "Inter",
        textAlign: "center" as const,
        ...element.style,
      },
    };

    set((state) => ({
      elements: [...state.elements, newElement],
      selectedElementIds: [newElement.id],
    }));
  },

  updateElement: (id, updates) => {
    set((state) => ({
      elements: state.elements.map((el) =>
        el.id === id ? { 
            ...el, 
            ...updates, 
            style: { ...el.style, ...updates.style },
            interactions: updates.interactions || el.interactions
        } : el
      ),
    }));
  },

  updateSelectedElements: (updates) => {
    const { selectedElementIds } = get();
    if (selectedElementIds.length === 0) return;
    
    set((state) => ({
      elements: state.elements.map((el) =>
        selectedElementIds.includes(el.id) 
          ? { ...el, ...updates, style: { ...el.style, ...updates.style } } 
          : el
      ),
    }));
  },

  duplicateElements: (ids) => {
    get().saveToHistory();
    set((state) => {
      const sourceElements = state.elements.filter(el => ids.includes(el.id));
      const duplicatedElements = sourceElements.map(el => ({
        ...el,
        id: `el-${crypto.randomUUID()}`,
        x: el.x + 20,
        y: el.y + 20,
        zIndex: state.elements.length + 1,
      }));
      
      return {
        elements: [...state.elements, ...duplicatedElements],
        selectedElementIds: duplicatedElements.map(el => el.id)
      };
    });
  },

  deleteElements: (ids) => {
    get().saveToHistory();
    set((state) => ({
      elements: state.elements.filter((el) => !ids.includes(el.id)),
      selectedElementIds: state.selectedElementIds.filter((id) => !ids.includes(id)),
    }));
  },

  setSelectedElements: (ids) => set({ selectedElementIds: ids }),

  reorderElements: (startIndex: number, endIndex: number) => {
    set((state) => {
      const newElements = [...state.elements];
      const [removed] = newElements.splice(startIndex, 1);
      if (removed) {
        newElements.splice(endIndex, 0, removed);
      }
      
      // Update z-index based on new order to keep rendering consistent
      return {
        elements: newElements.map((el, i) => ({
          ...el,
          zIndex: i
        }))
      };
    });
  },

  bringToFront: (id) => {
    set((state) => {
      const maxZ = Math.max(...state.elements.map(el => el.zIndex), 0);
      return {
        elements: state.elements.map(el => el.id === id ? { ...el, zIndex: maxZ + 1 } : el)
      };
    });
  },

  sendToBack: (id) => {
    set((state) => {
      const minZ = Math.min(...state.elements.map(el => el.zIndex), 0);
      return {
        elements: state.elements.map(el => el.id === id ? { ...el, zIndex: minZ - 1 } : el)
      };
    });
  },

  bringForward: (id) => {
    set((state) => {
      const sorted = [...state.elements].sort((a, b) => a.zIndex - b.zIndex);
      const idx = sorted.findIndex((el) => el.id === id);
      
      if (idx !== -1 && idx < sorted.length - 1) {
        const current = sorted[idx]!;
        const next = sorted[idx + 1]!;
        
        const currentZ = current.zIndex;
        const nextZ = next.zIndex;

        return {
          elements: state.elements.map((el) => {
            if (el.id === current.id) return { ...el, zIndex: nextZ };
            if (el.id === next.id) return { ...el, zIndex: currentZ };
            return el;
          }),
        };
      }
      return state;
    });
  },

  sendBackward: (id) => {
    set((state) => {
      const sorted = [...state.elements].sort((a, b) => a.zIndex - b.zIndex);
      const idx = sorted.findIndex((el) => el.id === id);
      
      if (idx > 0) {
        const current = sorted[idx]!;
        const prev = sorted[idx - 1]!;
        
        const currentZ = current.zIndex;
        const prevZ = prev.zIndex;

        return {
          elements: state.elements.map((el) => {
            if (el.id === current.id) return { ...el, zIndex: prevZ };
            if (el.id === prev.id) return { ...el, zIndex: currentZ };
            return el;
          }),
        };
      }
      return state;
    });
  },

  setCanvasConfig: (config) =>
    set((state) => ({
      canvasConfig: { ...state.canvasConfig, ...config },
    })),

  undo: () => {
    const { elements, history } = get();
    if (history.past.length === 0) return;

    const previous = history.past[history.past.length - 1];
    const newPast = history.past.slice(0, history.past.length - 1);

    set({
      elements: previous,
      history: {
        past: newPast,
        future: [elements, ...history.future],
      },
    });
  },

  redo: () => {
    const { elements, history } = get();
    if (history.future.length === 0) return;

    const next = history.future[0];
    const newFuture = history.future.slice(1);

    set({
      elements: next,
      history: {
        past: [...history.past, elements],
        future: newFuture,
      },
    });
  },
}));
