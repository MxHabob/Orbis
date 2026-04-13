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
    <div 
      className="flex-1 relative overflow-hidden certificate-editor-canvas" 
      style={{ backgroundColor: "#0c0c0e" }}
    >
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

    {/* Status Bar */}
    <div className="absolute bottom-6 left-1/2 -translate-x-1/2 glass-panel px-5 py-2.5 rounded-2xl text-[10px] font-black tracking-[0.1em] text-white/40 pointer-events-none flex items-center gap-4 uppercase">
        <div className="flex items-center gap-2">
            <span className="w-1.5 h-1.5 rounded-full bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]" />
            <span className="text-white/80">Active</span>
        </div>
        <div className="h-3 w-[1px] bg-white/10" />
        <span className="text-blue-400 font-bold">{Math.round(canvasConfig.zoom * 100)}%</span>
        <div className="h-3 w-[1px] bg-white/10" />
        <span>{elements.length} Objects</span>
        <div className="h-3 w-[1px] bg-white/10" />
        <span className="text-white/20 italic font-medium normal-case tracking-normal">Hold Space to Pan</span>
    </div>
   </div>
  );
}
