"use client";

import React, { useRef, useMemo } from "react";
import { TransformWrapper, TransformComponent } from "react-zoom-pan-pinch";
import { useEditorStore } from "@/store/certificate-editor";
import { EditorElement } from "./editor-element";
import { EditorSelectionManager } from "./editor-selection-manager";
import { EditorContextMenu } from "./editor-context-menu";
import { cn } from "@midday/ui/cn";

export function EditorCanvas() {
  const { elements, canvasConfig, setCanvasConfig, setSelectedElements } = useEditorStore();
  const canvasRef = useRef<HTMLDivElement>(null);
  const [isMiddleMouseDown, setIsMiddleMouseDown] = React.useState(false);

  const layoutConfig = useMemo(() => canvasConfig.layoutConfig || { rows: 1, columns: 1, gap: 0 }, [canvasConfig.layoutConfig]);
  const baseFrame = useMemo(() => elements.find(el => el.type === 'frame'), [elements]);
  
  const totalSlots = layoutConfig.rows * layoutConfig.columns;

  return (
    <div className="flex-1 relative overflow-hidden bg-[#0c0c0e]">
      {/* Precision Grid Layer */}
      <div 
        className="absolute inset-0 canvas-grid pointer-events-none opacity-20 transition-opacity duration-500"
        style={{ 
          backgroundSize: `${24 * (canvasConfig.zoom || 1)}px ${24 * (canvasConfig.zoom || 1)}px`,
          backgroundImage: `radial-gradient(hsl(var(--primary)) 1px, transparent 1px)`
        }} 
      />
      <TransformWrapper
        initialScale={0.5}
        minScale={0.02}
        maxScale={10}
        centerOnInit={true}
        onZoom={(ref) => setCanvasConfig({ zoom: ref.state.scale })}
        panning={{
          excluded: ["input", "button", "textarea"],
          activationKeys: isMiddleMouseDown ? [] : [" "], 
        }}
        limitToBounds={false}
        doubleClick={{ disabled: true }}
        wheel={{ step: 0.1 }}
      >
        <div 
          className="w-full h-full outline-none"
          tabIndex={0}
          onMouseDown={(e) => {
            if (e.button === 1) setIsMiddleMouseDown(true);
          }}
          onMouseUp={(e) => {
             if (e.button === 1) setIsMiddleMouseDown(false);
          }}
          onClick={(e) => {
             const target = e.target as HTMLElement;
             const isElement = target.closest(".editor-element-container");
             const isMoveable = target.closest(".moveable-control-box");
             const isToolbar = target.closest(".floating-toolbar");
             const isContext = target.closest(".context-menu-content");
             
             if (!isElement && !isMoveable && !isToolbar && !isContext) {
                setSelectedElements([]);
             }
          }}
        >
          <TransformComponent
              wrapperStyle={{ width: "100%", height: "100%", cursor: "crosshair" }}
              contentStyle={{
                minWidth: "100%",
                minHeight: "100%",
              }}
            >
              <EditorContextMenu>
                <div
                    ref={canvasRef}
                    id="infinite-canvas"
                    className="relative"
                    style={{
                      width: "10000px",
                      height: "10000px",
                      backgroundColor: "transparent", 
                      pointerEvents: "auto",
                      backgroundImage: canvasConfig.showGrid 
                        ? `radial-gradient(${canvasConfig.gridColor || 'rgba(255,255,255,0.03)'} 1px, transparent 1px)` 
                        : 'none',
                      backgroundSize: `${canvasConfig.gridSize || 20}px ${canvasConfig.gridSize || 20}px`,
                    }}
                >
                    {/* Master Elements */}
                    {elements.map((element) => (
                        <EditorElement key={element.id} element={element} />
                    ))}

                    {/* Non-intrusive Grid Overlay for Frame Division */}
                    {baseFrame && (layoutConfig.rows > 1 || layoutConfig.columns > 1) && (
                        <div 
                            className="absolute pointer-events-none rounded-[inherit]"
                            style={{
                                left: baseFrame.x,
                                top: baseFrame.y,
                                width: baseFrame.width,
                                height: baseFrame.height,
                                zIndex: baseFrame.zIndex + 0.5,
                                backgroundImage: `
                                    linear-gradient(to right, hsl(var(--primary) / 0.15) 1px, transparent 1px),
                                    linear-gradient(to bottom, hsl(var(--primary) / 0.15) 1px, transparent 1px)
                                `,
                                backgroundSize: `${(baseFrame.width + layoutConfig.gap) / layoutConfig.columns}px ${(baseFrame.height + layoutConfig.gap) / layoutConfig.rows}px`,
                                border: '1px solid hsl(var(--primary) / 0.2)',
                            }}
                        />
                    )}
                    
                    <EditorSelectionManager containerRef={canvasRef} />
                </div>
              </EditorContextMenu>
        </TransformComponent>
      </div>
    </TransformWrapper>
   </div>
  );
}
