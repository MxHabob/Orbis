"use client";

import React, { useRef } from "react";
import { TransformWrapper, TransformComponent } from "react-zoom-pan-pinch";
import { useEditorStore } from "@/store/certificate-editor";
import { EditorElement } from "./editor-element";
import { EditorSelectionManager } from "./editor-selection-manager";
import { EditorContextMenu } from "./editor-context-menu";

export function EditorCanvas() {
  const { elements, canvasConfig, setCanvasConfig, setSelectedElements } = useEditorStore();
  const canvasRef = useRef<HTMLDivElement>(null);
  const [isMiddleMouseDown, setIsMiddleMouseDown] = React.useState(false);

  return (
    <div className="flex-1 relative overflow-hidden bg-background">
      {/* Precision Grid Layer */}
      <div 
        className="absolute inset-0 canvas-grid pointer-events-none opacity-40 transition-opacity duration-500"
        style={{ 
          backgroundSize: `${24 * (canvasConfig.zoom || 1)}px ${24 * (canvasConfig.zoom || 1)}px`,
          backgroundImage: `radial-gradient(hsl(var(--border)) 1px, transparent 1px)`
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
                      backgroundColor: "#0c0c0e",
                      pointerEvents: "auto",
                      backgroundImage: canvasConfig.showGrid 
                        ? `radial-gradient(${canvasConfig.gridColor || 'rgba(255,255,255,0.05)'} 1px, transparent 1px)` 
                        : 'none',
                      backgroundSize: `${canvasConfig.gridSize || 20}px ${canvasConfig.gridSize || 20}px`,
                    }}
                >
                    {elements.map((element) => (
                        <EditorElement key={element.id} element={element} />
                    ))}
                    
                    <EditorSelectionManager containerRef={canvasRef} />
                </div>
              </EditorContextMenu>
        </TransformComponent>
      </div>
    </TransformWrapper>
   </div>
  );
}
