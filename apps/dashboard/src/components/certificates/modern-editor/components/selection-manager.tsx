"use client";

import React, { useEffect, useState, useRef } from "react";
import Moveable from "react-moveable";
import Selecto from "react-selecto";
import { useEditorStore } from "../store/use-editor-store";

interface Props {
  containerRef: React.RefObject<HTMLDivElement | null>;
}

export function SelectionManager({ containerRef }: Props) {
  const { 
    elements, 
    selectedElementIds, 
    setSelectedElements, 
    updateElement, 
    deleteElements, 
    duplicateElements 
  } = useEditorStore();

  const [targets, setTargets] = useState<HTMLElement[]>([]);
  const moveableRef = useRef<Moveable>(null);

  // Keyboard Shortcuts (Delete, Duplicate)
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (selectedElementIds.length === 0) return;
      
      if (e.key === "Delete" || e.key === "Backspace") {
        deleteElements(selectedElementIds);
      }

      if ((e.metaKey || e.ctrlKey) && e.key === "d") {
        e.preventDefault();
        duplicateElements(selectedElementIds);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [selectedElementIds, deleteElements, duplicateElements]);

  // Sync targets with selected IDs
  useEffect(() => {
    const selectedElements = elements.filter(el => selectedElementIds.includes(el.id));
    const newTargets = selectedElements
      .map(el => document.querySelector(`[data-element-id="${el.id}"]`))
      .filter(Boolean) as HTMLElement[];
    setTargets(newTargets);
  }, [selectedElementIds, elements]);

  return (
    <>
      <Moveable
        ref={moveableRef}
        target={targets}
        container={containerRef.current}
        draggable={true}
        resizable={true}
        rotatable={true}
        snappable={true}
        keepRatio={false}
        throttleDrag={1}
        edge={false}
        zoom={1}
        origin={false}
        padding={{ left: 0, top: 0, right: 0, bottom: 0 }}
        
        // Element Drag Logic
        onDrag={({ target, beforeTranslate }) => {
          const id = target.getAttribute("data-element-id");
          const el = elements.find(e => e.id === id);
          if (el) {
             // Visual update (instant feedback)
             target.style.transform = `translate(${beforeTranslate[0]}px, ${beforeTranslate[1]}px) rotate(${el.rotation}deg)`;
          }
        }}
        onDragEnd={({ target, lastEvent }) => {
          const id = target.getAttribute("data-element-id");
          if (id && lastEvent) {
             const el = elements.find(e => e.id === id);
             if (el) {
                // translate[0/1] is the logical distance moved
                updateElement(id, {
                  x: Math.round(el.x + lastEvent.beforeTranslate[0]),
                  y: Math.round(el.y + lastEvent.beforeTranslate[1]),
                });
             }
             // Clear the temporary transform to let React render the new left/top
             target.style.transform = "";
          }
        }}
        
        // Resize Logic
        onResize={({ target, width, height, drag }) => {
          const id = target.getAttribute("data-element-id");
          const el = elements.find(e => e.id === id);
          if (el) {
            target.style.width = `${width}px`;
            target.style.height = `${height}px`;
            target.style.transform = `translate(${drag.beforeTranslate[0]}px, ${drag.beforeTranslate[1]}px) rotate(${el.rotation}deg)`;
          }
        }}
        onResizeEnd={({ target, lastEvent }) => {
          const id = target.getAttribute("data-element-id");
          if (id && lastEvent) {
             const el = elements.find(e => e.id === id);
             if (el) {
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
        
        // Custom Click/Select Logic
        onClickGroup={({ inputEvent }) => {
            // Logic for clicking a group of elements
        }}
      />

      {/* Marquee Selection Tool (Figma Style) */}
      <Selecto
        dragContainer={containerRef.current}
        selectableTargets={[".editor-element-container"]}
        hitRate={0}
        selectByClick={true}
        selectFromInside={false}
        toggleContinueSelect={["shift"]}
        ratio={0}
        onDragStart={(e) => {
            const target = e.inputEvent.target as HTMLElement;
            // Don't start selection if clicking on moveable controls or an element directly
            if (
                moveableRef.current?.isMoveableElement(target) ||
                targets.some(t => t === target || t.contains(target))
            ) {
                e.stop();
            }
        }}
        onSelect={(e) => {
            const ids = e.selected
                .map(el => el.getAttribute('data-element-id'))
                .filter(Boolean) as string[];
            
            setSelectedElements(ids);
        }}
        // Styling the marquee box
        className="selecto-selection"
      />

      <style jsx global>{`
        .selecto-selection {
          background: rgba(12, 140, 233, 0.1) !important;
          border: 1px solid #0c8ce9 !important;
          border-radius: 2px !important;
        }
      `}</style>
    </>
  );
}
