import { create } from "zustand";
import { subscribeWithSelector } from "zustand/middleware";
import type { EditorElement, EditorState } from "@/types/certificate-editor";

interface EditorActions {
  setName: (name: string) => void;
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

  // Grouping
  groupElements: () => void;
  ungroupElements: (id: string) => void;

  // Clipboard
  copySelection: () => void;
  pasteSelection: () => void;
}

export const useEditorStore = create<EditorState & EditorActions>()(
  subscribeWithSelector((set, get) => ({
    name: "Untitled Template",
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

    setName: (name) => set({ name }),
    setMode: (mode) => set({ mode }),
    setElements: (elements) => set({ elements }),

    saveToHistory: () => {
      const { elements, history } = get();
      // Skip if exactly the same as last past state
      const lastPast = history.past[history.past.length - 1];
      if (JSON.stringify(lastPast) === JSON.stringify(elements)) return;

      set({
        history: {
          past: [...history.past, elements].slice(-100), // Increased history limit
          future: [],
        },
      });
    },

    addElement: (element) => {
      get().saveToHistory();

      const isFrame = element.type === "frame";
      const defaultWidth = isFrame ? 800 : 400;
      const defaultHeight = isFrame ? 600 : 80;

      const newElement: EditorElement = {
        id: `el-${crypto.randomUUID()}`,
        name:
          element.name ||
          (element.type === "variable"
            ? `Variable: ${element.variableKey}`
            : `New ${element.type || "text"}`),
        type: element.type || "text",
        x: element.x ?? 5000 - (element.width || defaultWidth) / 2,
        y: element.y ?? 5000 - (element.height || defaultHeight) / 2,
        width: element.width ?? defaultWidth,
        height: element.height ?? defaultHeight,
        rotation: 0,
        opacity: 1,
        zIndex: get().elements.length,
        visible: true,
        locked: false,
        interactions: [],
        ...element,
        style: {
          fontSize: 32,
          color: isFrame ? "transparent" : "#000000",
          backgroundColor: isFrame ? "#ffffff" : "transparent",
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
      set((state) => {
        const targetElement = state.elements.find((el) => el.id === id);
        if (!targetElement) return state;

        const deltaX =
          updates.x !== undefined ? updates.x - targetElement.x : 0;
        const deltaY =
          updates.y !== undefined ? updates.y - targetElement.y : 0;

        // Recursive function to update children if it's a group
        const updateRecursive = (
          elements: EditorElement[],
          elementId: string,
          dx: number,
          dy: number,
        ): EditorElement[] => {
          let newElements = [...elements];
          const children = newElements.filter(
            (el) => el.parentId === elementId,
          );

          children.forEach((child) => {
            newElements = newElements.map((el) =>
              el.id === child.id ? { ...el, x: el.x + dx, y: el.y + dy } : el,
            );
            // Recursively update descendants
            if (child.type === "group") {
              newElements = updateRecursive(newElements, child.id, dx, dy);
            }
          });

          return newElements;
        };

        let newElements = state.elements.map((el) =>
          el.id === id
            ? {
                ...el,
                ...updates,
                style: { ...el.style, ...updates.style },
                interactions: updates.interactions || el.interactions,
              }
            : el,
        );

        if (targetElement.type === "group" && (deltaX !== 0 || deltaY !== 0)) {
          newElements = updateRecursive(newElements, id, deltaX, deltaY);
        }

        return { elements: newElements };
      });
    },

    updateSelectedElements: (updates) => {
      const { selectedElementIds } = get();
      if (selectedElementIds.length === 0) return;

      set((state) => ({
        elements: state.elements.map((el) => {
          if (selectedElementIds.includes(el.id)) {
            // Smart update: merge deep styles correctly
            return {
              ...el,
              ...updates,
              style: { ...el.style, ...updates.style },
            };
          }
          return el;
        }),
      }));
    },

    duplicateElements: (ids) => {
      get().saveToHistory();
      set((state) => {
        const sourceElements = state.elements.filter((el) =>
          ids.includes(el.id),
        );
        const duplicatedElements = sourceElements.map((el) => ({
          ...el,
          id: `el-${crypto.randomUUID()}`,
          x: el.x + 20,
          y: el.y + 20,
          zIndex: Math.max(...state.elements.map((e) => e.zIndex), 0) + 1,
        }));

        return {
          elements: [...state.elements, ...duplicatedElements],
          selectedElementIds: duplicatedElements.map((el) => el.id),
        };
      });
    },

    deleteElements: (ids) => {
      if (ids.length === 0) return;
      get().saveToHistory();
      set((state) => ({
        elements: state.elements.filter((el) => !ids.includes(el.id)),
        selectedElementIds: state.selectedElementIds.filter(
          (id) => !ids.includes(id),
        ),
      }));
    },

    setSelectedElements: (ids) => {
      // Only set if different to avoid re-renders
      const currentSelected = get().selectedElementIds;
      if (JSON.stringify(currentSelected) === JSON.stringify(ids)) return;
      set({ selectedElementIds: ids });
    },

    reorderElements: (startIndex: number, endIndex: number) => {
      set((state) => {
        const newElements = [...state.elements];
        const [removed] = newElements.splice(startIndex, 1);
        if (removed) {
          newElements.splice(endIndex, 0, removed);
        }

        return {
          elements: newElements.map((el, i) => ({
            ...el,
            zIndex: i,
          })),
        };
      });
    },

    bringToFront: (id) => {
      set((state) => {
        const sorted = [...state.elements].sort((a, b) => a.zIndex - b.zIndex);
        const element = sorted.find((el) => el.id === id);
        if (!element) return state;

        const otherElements = sorted.filter((el) => el.id !== id);
        const newElements = [...otherElements, element];

        return {
          elements: newElements.map((el, i) => ({ ...el, zIndex: i })),
        };
      });
    },

    sendToBack: (id) => {
      set((state) => {
        const sorted = [...state.elements].sort((a, b) => a.zIndex - b.zIndex);
        const element = sorted.find((el) => el.id === id);
        if (!element) return state;

        const otherElements = sorted.filter((el) => el.id !== id);
        const newElements = [element, ...otherElements];

        return {
          elements: newElements.map((el, i) => ({ ...el, zIndex: i })),
        };
      });
    },

    bringForward: (id) => {
      set((state) => {
        const sorted = [...state.elements].sort((a, b) => a.zIndex - b.zIndex);
        const idx = sorted.findIndex((el) => el.id === id);

        if (idx !== -1 && idx < sorted.length - 1) {
          const newSorted = [...sorted];
          [newSorted[idx], newSorted[idx + 1]] = [
            newSorted[idx + 1]!,
            newSorted[idx]!,
          ];

          return {
            elements: newSorted.map((el, i) => ({ ...el, zIndex: i })),
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
          const newSorted = [...sorted];
          [newSorted[idx], newSorted[idx - 1]] = [
            newSorted[idx - 1]!,
            newSorted[idx]!,
          ];

          return {
            elements: newSorted.map((el, i) => ({ ...el, zIndex: i })),
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

      const previous = history.past[history.past.length - 1]!;
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

      const next = history.future[0]!;
      const newFuture = history.future.slice(1);

      set({
        elements: next,
        history: {
          past: [...history.past, elements],
          future: newFuture,
        },
      });
    },

    copySelection: () => {
      const { elements, selectedElementIds } = get();
      const selected = elements.filter((el) =>
        selectedElementIds.includes(el.id),
      );
      if (selected.length > 0) {
        localStorage.setItem("editor_clipboard", JSON.stringify(selected));
      }
    },

    pasteSelection: () => {
      const clipboard = localStorage.getItem("editor_clipboard");
      if (!clipboard) return;

      try {
        const parsed = JSON.parse(clipboard) as EditorElement[];
        get().saveToHistory();

        const pasted = parsed.map((el) => ({
          ...el,
          id: `el-${crypto.randomUUID()}`,
          x: el.x + 40,
          y: el.y + 40,
          zIndex: get().elements.length + 1,
        }));

        set((state) => ({
          elements: [...state.elements, ...pasted],
          selectedElementIds: pasted.map((el) => el.id),
        }));
      } catch (e) {
        console.error("Paste failed", e);
      }
    },

    groupElements: () => {
      const { elements, selectedElementIds } = get();
      if (selectedElementIds.length < 2) return;

      get().saveToHistory();

      const selectedElements = elements.filter((el) =>
        selectedElementIds.includes(el.id),
      );

      // Calculate bounding box
      const minX = Math.min(...selectedElements.map((el) => el.x));
      const minY = Math.min(...selectedElements.map((el) => el.y));
      const maxX = Math.max(...selectedElements.map((el) => el.x + el.width));
      const maxY = Math.max(...selectedElements.map((el) => el.y + el.height));

      const groupId = `group-${crypto.randomUUID()}`;
      const newGroup: EditorElement = {
        id: groupId,
        name: "New Group",
        type: "group",
        x: minX,
        y: minY,
        width: maxX - minX,
        height: maxY - minY,
        rotation: 0,
        opacity: 1,
        zIndex: Math.max(...elements.map((e) => e.zIndex)) + 1,
        visible: true,
        locked: false,
        style: {
          backgroundColor: "transparent",
        },
      };

      set((state) => ({
        elements: [
          ...state.elements.map((el) =>
            selectedElementIds.includes(el.id)
              ? { ...el, parentId: groupId }
              : el,
          ),
          newGroup,
        ],
        selectedElementIds: [groupId],
      }));
    },

    ungroupElements: (id) => {
      const { elements } = get();
      const group = elements.find((el) => el.id === id && el.type === "group");
      if (!group) return;

      get().saveToHistory();

      set((state) => ({
        elements: state.elements
          .filter((el) => el.id !== id)
          .map((el) =>
            el.parentId === id ? { ...el, parentId: group.parentId } : el,
          ),
        selectedElementIds: state.elements
          .filter((el) => el.parentId === id)
          .map((el) => el.id),
      }));
    },
  })),
);
