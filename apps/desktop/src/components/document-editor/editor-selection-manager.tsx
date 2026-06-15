import type React from "react";
import { useEffect, useRef, useState } from "react";
import Moveable from "react-moveable";
import Selecto from "react-selecto";
import { useEditorStore } from "@/store/document-editor";

interface Props {
  containerRef: React.RefObject<HTMLDivElement | null>;
}

export function EditorSelectionManager({ containerRef }: Props) {
  const {
    elements,
    selectedElementIds,
    setSelectedElements,
    updateElement,
    deleteElements,
    duplicateElements,
    groupElements,
    ungroupElements,
    undo,
    redo,
    saveToHistory,
  } = useEditorStore();

  const [targets, setTargets] = useState<HTMLElement[]>([]);
  const moveableRef = useRef<Moveable>(null);

  // Shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (["INPUT", "TEXTAREA"].includes((e.target as HTMLElement).tagName))
        return;

      const step = e.shiftKey ? 10 : 1;

      // Undo/Redo Shortcuts
      if ((e.metaKey || e.ctrlKey) && e.key === "z") {
        e.preventDefault();
        if (e.shiftKey) {
          redo();
        } else {
          undo();
        }
        return;
      }

      if ((e.metaKey || e.ctrlKey) && e.key === "y") {
        e.preventDefault();
        redo();
        return;
      }

      if (selectedElementIds.length === 0) return;

      // Grouping Shortcuts
      if ((e.metaKey || e.ctrlKey) && e.key === "g") {
        e.preventDefault();
        if (e.shiftKey) {
          // Ungroup each selected element if it's a group
          selectedElementIds.forEach((id) => {
            const el = elements.find((e) => e.id === id);
            if (el?.type === "group") ungroupElements(id);
          });
        } else {
          groupElements();
        }
        return;
      }

      if (e.key === "Delete" || e.key === "Backspace") {
        deleteElements(selectedElementIds);
      }

      if ((e.metaKey || e.ctrlKey) && e.key === "d") {
        e.preventDefault();
        duplicateElements(selectedElementIds);
      }

      if (["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].includes(e.key)) {
        e.preventDefault();
        const updates: { x?: number; y?: number } = {};
        if (e.key === "ArrowUp") updates.y = -step;
        if (e.key === "ArrowDown") updates.y = step;
        if (e.key === "ArrowLeft") updates.x = -step;
        if (e.key === "ArrowRight") updates.x = step;

        selectedElementIds.forEach((id) => {
          const el = elements.find((e) => e.id === id);
          if (el) {
            updateElement(id, {
              x: el.x + (updates.x || 0),
              y: el.y + (updates.y || 0),
            });
          }
        });
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [
    selectedElementIds,
    elements,
    deleteElements,
    duplicateElements,
    updateElement,
  ]);

  useEffect(() => {
    const newTargets = selectedElementIds
      .map((id) => document.querySelector(`[data-element-id="${id}"]`))
      .filter(Boolean) as HTMLElement[];
    setTargets(newTargets);
  }, [selectedElementIds]);

  return (
    <>
      <Moveable
        ref={moveableRef}
        target={targets}
        container={containerRef.current}
        className="document-moveable-controls"
        draggable={true}
        resizable={true}
        rotatable={true}
        snappable={true}
        snapThreshold={5}
        elementGuidelines={
          elements
            .filter((el) => !selectedElementIds.includes(el.id))
            .map((el) => document.querySelector(`[data-element-id="${el.id}"]`))
            .filter(Boolean) as HTMLElement[]
        }
        onDrag={({ target, transform }) => {
          target.style.transform = transform;
        }}
        onDragEnd={({ target, isDrag, lastEvent }) => {
          if (!isDrag || !lastEvent) return;
          const id = target.getAttribute("data-element-id");
          if (id) {
            const el = elements.find((e) => e.id === id);
            if (el) {
              saveToHistory();
              updateElement(id, {
                x: Math.round(el.x + lastEvent.beforeTranslate[0]),
                y: Math.round(el.y + lastEvent.beforeTranslate[1]),
              });
            }
          }
          target.style.transform = "";
        }}
        onDragGroup={({ events }) => {
          events.forEach(({ target, transform }) => {
            target.style.transform = transform;
          });
        }}
        onDragGroupEnd={({ events }) => {
          saveToHistory();
          events.forEach(({ target, lastEvent }) => {
            const id = target.getAttribute("data-element-id");
            if (id && lastEvent) {
              const el = elements.find((e) => e.id === id);
              if (el) {
                updateElement(id, {
                  x: Math.round(el.x + lastEvent.beforeTranslate[0]),
                  y: Math.round(el.y + lastEvent.beforeTranslate[1]),
                });
              }
            }
            target.style.transform = "";
          });
        }}
        onResize={({ target, width, height, drag }) => {
          target.style.width = `${width}px`;
          target.style.height = `${height}px`;
          target.style.transform = drag.transform;
        }}
        onResizeEnd={({ target, lastEvent }) => {
          const id = target.getAttribute("data-element-id");
          if (id && lastEvent) {
            const el = elements.find((e) => e.id === id);
            if (el) {
              saveToHistory();
              updateElement(id, {
                width: Math.round(parseFloat(target.style.width)),
                height: Math.round(parseFloat(target.style.height)),
                x: Math.round(el.x + lastEvent.drag.beforeTranslate[0]),
                y: Math.round(el.y + lastEvent.drag.beforeTranslate[1]),
              });
            }
            target.style.transform = "";
          }
        }}
        onResizeGroup={({ events }) => {
          events.forEach(({ target, width, height, drag }) => {
            target.style.width = `${width}px`;
            target.style.height = `${height}px`;
            target.style.transform = drag.transform;
          });
        }}
        onResizeGroupEnd={({ events }) => {
          saveToHistory();
          events.forEach(({ target, lastEvent }) => {
            const id = target.getAttribute("data-element-id");
            if (id && lastEvent) {
              const el = elements.find((e) => e.id === id);
              if (el) {
                updateElement(id, {
                  width: Math.round(parseFloat(target.style.width)),
                  height: Math.round(parseFloat(target.style.height)),
                  x: Math.round(el.x + lastEvent.drag.beforeTranslate[0]),
                  y: Math.round(el.y + lastEvent.drag.beforeTranslate[1]),
                });
              }
            }
            target.style.transform = "";
          });
        }}
        onRotate={({ target, transform }) => {
          target.style.transform = transform;
        }}
        onRotateEnd={({ target, lastEvent }) => {
          const id = target.getAttribute("data-element-id");
          if (id && lastEvent) {
            saveToHistory();
            updateElement(id, { rotation: lastEvent.rotation });
          }
          target.style.transform = "";
        }}
        onRotateGroup={({ events }) => {
          events.forEach(({ target, transform }) => {
            target.style.transform = transform;
          });
        }}
        onRotateGroupEnd={({ events }) => {
          saveToHistory();
          events.forEach(({ target, lastEvent }) => {
            const id = target.getAttribute("data-element-id");
            if (id && lastEvent) {
              updateElement(id, { rotation: lastEvent.rotation });
            }
            target.style.transform = "";
          });
        }}
      />

      <Selecto
        dragContainer={containerRef.current}
        selectableTargets={[".editor-element-container"]}
        hitRate={0}
        selectByClick={false}
        selectFromInside={false}
        toggleContinueSelect={[]}
        ratio={0}
        onDragStart={(e) => {
          const target = e.inputEvent.target as HTMLElement;
          if (
            moveableRef.current?.isMoveableElement(target) ||
            targets.some((t) => t === target || t.contains(target))
          ) {
            e.stop();
          }
        }}
        onSelect={(e) => {
          const findRootId = (id: string): string => {
            const el = elements.find((e) => e.id === id);
            if (el?.parentId) return findRootId(el.parentId);
            return id;
          };

          const ids = Array.from(
            new Set(
              e.selected
                .map((el) => el.getAttribute("data-element-id"))
                .filter(Boolean)
                .map((id) => findRootId(id!)),
            ),
          );

          setSelectedElements(ids);
        }}
        className="selecto-selection"
      />
    </>
  );
}
