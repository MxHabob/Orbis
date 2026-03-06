"use client";

import { memo, useRef, useState, useEffect, useCallback, useMemo } from "react";
import { TransformWrapper, TransformComponent } from "react-zoom-pan-pinch";
import { Button } from "@midday/ui/button";
import { ZoomIn, ZoomOut } from "lucide-react";
import { cn } from "@midday/ui/cn";
import { DraggableElement } from "./draggable-element";
import type { EditorElement, PaperSize } from "./types";
import Moveable from "react-moveable";
import Selecto from "react-selecto";

interface CanvasProps {
  canvasRef: any;
  orientation: "portrait" | "landscape";
  paperSize: PaperSize;
  canvasWidth: number;
  canvasHeight: number;
  isPreview: boolean;
  backgroundImage: string;
  elements: EditorElement[];
  selectedElementId: string | null;
  setSelectedElementId: (id: string | null) => void;
  previewData: any;
  students: any;
  previewStudentId: string | null;
  updateElement: (id: string, updates: Partial<EditorElement>) => void;
  onDelete: (id: string) => void;
  certificateType: string;
  month: number;
}

export const Canvas = memo(({
  canvasRef,
  orientation,
  paperSize,
  canvasWidth,
  canvasHeight,
  isPreview,
  backgroundImage,
  elements,
  selectedElementId,
  setSelectedElementId,
  previewData,
  students,
  previewStudentId,
  updateElement,
  onDelete,
  certificateType,
  month,
}: CanvasProps) => {
  const [scale, setScale] = useState(1);
  const moveableRef = useRef<Moveable>(null);
  const [targets, setTargets] = useState<HTMLElement[]>([]);

  useEffect(() => {
    if (selectedElementId) {
      const el = document.getElementById(selectedElementId);
      if (el) setTargets([el]);
      else setTargets([]);
    } else {
      setTargets([]);
    }
  }, [selectedElementId, elements]);

  const [isInteracting, setIsInteracting] = useState(false);

  // 1. Unified State Synchronization
  const syncElement = useCallback((id: string, lastEvent: any) => {
    setIsInteracting(false);
    if (!id || !lastEvent) return;

    const updates: Partial<EditorElement> = {};
    
    // Drag data
    if ("left" in lastEvent) {
      updates.x = (lastEvent.left / canvasWidth) * 100;
      updates.y = (lastEvent.top / canvasHeight) * 100;
    }
    
    // Resize data
    if ("width" in lastEvent) {
      const el = elements.find(e => e.id === id);
      updates.width = (lastEvent.width / canvasWidth) * 100;
      
      // If it's a text/variable, we scale the font size
      if (el && (el.type === "text" || el.type === "variable") && el.fontSize) {
          const ratio = lastEvent.width / (lastEvent.lastWidth || lastEvent.width);
          // Only update font size if it's a significant change to avoid jitter
          if (ratio !== 1) {
             const newFontSize = Math.round(el.fontSize * (lastEvent.width / ((el.width || 20) / 100 * canvasWidth)));
             updates.fontSize = newFontSize;
          }
      }

      updates.height = lastEvent.height;
      if (lastEvent.drag) {
        updates.x = (lastEvent.drag.left / canvasWidth) * 100;
        updates.y = (lastEvent.drag.top / canvasHeight) * 100;
      }
    }
    
    // Rotation data
    if ("rotate" in lastEvent) {
        updates.rotation = lastEvent.rotate;
    }

    updateElement(id, updates);
  }, [canvasWidth, canvasHeight, updateElement]);

  // 2. Memoized Guidelines for Performance
  const elementGuidelines = useMemo(() => 
    elements
      .filter(el => el.id !== selectedElementId)
      .map(el => document.getElementById(el.id))
      .filter(Boolean) as HTMLElement[],
    [elements, selectedElementId]
  );

  // 3. Professional Keyboard Support
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!selectedElementId || isPreview) return;
      
      const step = e.shiftKey ? 5 : 1; // Faster nudge with Shift
      const el = elements.find(prev => prev.id === selectedElementId);
      if (!el) return;

      if (["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"].includes(e.key)) {
        e.preventDefault();
        const pxStepX = (step / canvasWidth) * 100;
        const pxStepY = (step / canvasHeight) * 100;
        
        updateElement(selectedElementId, {
          x: el.x + (e.key === "ArrowRight" ? pxStepX : e.key === "ArrowLeft" ? -pxStepX : 0),
          y: el.y + (e.key === "ArrowDown" ? pxStepY : e.key === "ArrowUp" ? -pxStepY : 0),
        });
      }

      if (e.key === "Delete" || e.key === "Backspace") {
        if ((e.target as HTMLElement).tagName !== "INPUT") {
            onDelete(selectedElementId);
        }
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [selectedElementId, elements, updateElement, onDelete, canvasWidth, canvasHeight, isPreview]);

  return (
    <div className="flex-1 overflow-hidden p-8 flex items-center justify-center relative bg-[#1a1a1d] dark:bg-[#0c0c0e]">
      <Selecto
        dragContainer={".canvas-container"}
        selectableTargets={[".target"]}
        hitRate={0}
        selectByClick={true}
        selectFromInside={false}
        toggleContinueSelect={["shift"]}
        onSelectStart={() => setIsInteracting(true)}
        onSelectEnd={() => setIsInteracting(false)}
        onSelect={(e: any) => {
            if (isPreview) return;
            const target = e.added[0];
            if (target) {
                const id = target.getAttribute("data-id");
                setSelectedElementId(id);
            } else if (e.isClick) {
                setSelectedElementId(null);
            }
        }}
      />

      <TransformWrapper
        key={`${orientation}-${paperSize}`}
        initialScale={orientation === "landscape" ? 0.45 : 0.6}
        minScale={0.1}
        maxScale={4}
        centerOnInit
        onTransformed={(e) => setScale(e.state.scale)}
        panning={{ 
            disabled: isInteracting,
            excluded: ["target", "moveable-control", "moveable-line", "moveable-rotation-control"]
        }}
        limitToBounds={false}
      >
        {({ zoomIn, zoomOut, resetTransform }) => (
          <>
            {/* Zoom Controls — Redesigned */}
            <div className="absolute bottom-4 right-4 z-30 flex items-center gap-0.5 backdrop-blur-xl bg-black/60 dark:bg-white/10 border border-white/10 dark:border-white/[0.08] rounded-2xl shadow-[0_8px_32px_rgba(0,0,0,0.4)] py-1 px-1.5 transition-all duration-300 hover:bg-black/70 dark:hover:bg-white/15">
              <Button 
                variant="ghost" 
                size="icon" 
                className="h-7 w-7 rounded-xl text-white/70 hover:text-white hover:bg-white/10 transition-all duration-200" 
                onClick={() => zoomOut()}
              >
                <ZoomOut className="w-3.5 h-3.5" />
              </Button>
              <button 
                className="text-[11px] font-semibold tracking-wide px-2.5 min-w-[48px] text-center text-white/90 hover:text-white hover:bg-white/10 rounded-xl py-1 transition-all duration-200 font-mono"
                onClick={() => resetTransform()}
              >
                {(scale * 100).toFixed(0)}%
              </button>
              <Button 
                variant="ghost" 
                size="icon" 
                className="h-7 w-7 rounded-xl text-white/70 hover:text-white hover:bg-white/10 transition-all duration-200" 
                onClick={() => zoomIn()}
              >
                <ZoomIn className="w-3.5 h-3.5" />
              </Button>
            </div>

            {/* Preview Mode Indicator */}
            {isPreview && (
              <div className="absolute top-4 left-1/2 -translate-x-1/2 z-30 flex items-center gap-2 px-4 py-2 rounded-full bg-emerald-500/15 border border-emerald-500/20 backdrop-blur-xl">
                <div className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
                <span className="text-[11px] font-semibold text-emerald-300 tracking-wide">PREVIEW MODE</span>
              </div>
            )}

            <div className="w-full h-full canvas-container flex items-center justify-center">
              <TransformComponent wrapperClass="!w-full !h-full" contentClass="!w-full !h-full flex items-center justify-center">
                <div
                  ref={canvasRef}
                  className={cn(
                    "relative bg-white shadow-[0_20px_60px_rgba(0,0,0,0.15)] origin-center transition-all duration-500 ease-out-expo", 
                    !isPreview ? "ring-1 ring-black/5" : "overflow-hidden"
                  )}
                  style={{ 
                    width: canvasWidth, 
                    height: canvasHeight,
                  }}
                >
                  {/* Dot Grid Background */}
                  {!isPreview && (
                    <div 
                      className="absolute inset-0 pointer-events-none opacity-[0.03]" 
                      style={{ 
                        backgroundImage: `radial-gradient(#000 1px, transparent 1px)`, 
                        backgroundSize: '20px 20px' 
                      }} 
                    />
                  )}

                  {/* Center crosshair guides — editor only */}
                  {!isPreview && (
                    <>
                      <div className="absolute left-1/2 top-0 bottom-0 w-px bg-blue-500/[0.06] pointer-events-none" />
                      <div className="absolute top-1/2 left-0 right-0 h-px bg-blue-500/[0.06] pointer-events-none" />
                    </>
                  )}

                  {backgroundImage && (
                    <img src={backgroundImage} className="absolute inset-0 w-full h-full object-cover pointer-events-none" alt="Background" />
                  )}

                  <div className="absolute inset-0">
                        {elements.map((el) => (
                          <DraggableElement
                            key={el.id}
                            el={el}
                            isSelected={selectedElementId === el.id}
                            isPreview={isPreview}
                            content={isPreview && el.variableKey 
                              ? (previewData[el.variableKey.replace(/[{}]/g, "") as keyof typeof previewData] || el.content)
                              : el.content}
                            onSelect={setSelectedElementId}
                            onDelete={onDelete}
                            students={students}
                            previewStudentId={previewStudentId}
                            certificateType={certificateType}
                            month={month}
                          />
                        ))}
                  </div>

                  {!isPreview && (
                    <Moveable
                        ref={moveableRef}
                        target={targets}
                        draggable={true}
                        resizable={true}
                        rotatable={true}
                        snappable={true}
                        keepRatio={false}
                        zoom={1 / scale}
                        origin={false}
                        padding={{ left: 0, top: 0, right: 0, bottom: 0 }}
                        className="moveable-canva"
                        bounds={{ left: 0, top: 0, right: canvasWidth, bottom: canvasHeight }}
                        
                        // Guidelines & Snapping
                        verticalGuidelines={[0, canvasWidth / 4, canvasWidth / 2, canvasWidth * 3/4, canvasWidth]}
                        horizontalGuidelines={[0, canvasHeight / 4, canvasHeight / 2, canvasHeight * 3/4, canvasHeight]}
                        elementGuidelines={elementGuidelines}
                        snapThreshold={5}
                        
                        // Handlers
                        onDragStart={() => setIsInteracting(true)}
                        onDrag={e => e.target.style.cssText += `left:${e.left}px;top:${e.top}px;`}
                        onDragEnd={e => syncElement(e.target.getAttribute("data-id")!, e.lastEvent)}
                        
                        onResizeStart={() => setIsInteracting(true)}
                        onResize={e => {
                            e.target.style.width = `${e.width}px`;
                            e.target.style.height = `${e.height}px`;
                            e.target.style.transform = e.drag.transform;
                            
                            // Real-time font size feedback
                            const id = e.target.getAttribute("data-id");
                            const el = elements.find(item => item.id === id);
                            if (el && (el.type === "text" || el.type === "variable")) {
                                const ratio = e.width / (el.width ? (el.width / 100 * canvasWidth) : (e.target as HTMLElement).offsetWidth);
                                const newSize = Math.round((el.fontSize || 18) * ratio);
                                const textEl = e.target.querySelector('.text-content') as HTMLElement;
                                if (textEl) textEl.style.fontSize = `${newSize}px`;
                            }
                        }}
                        onResizeEnd={e => syncElement(e.target.getAttribute("data-id")!, e.lastEvent)}
                        
                        onRotateStart={() => setIsInteracting(true)}
                        onRotate={e => e.target.style.transform = e.drag.transform}
                        onRotateEnd={e => syncElement(e.target.getAttribute("data-id")!, e.lastEvent)}
                    />
                  )}
                </div>
              </TransformComponent>
            </div>
          </>
        )}
      </TransformWrapper>
    </div>
  );
});

Canvas.displayName = "Canvas";
