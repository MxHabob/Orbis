"use client";

import React, { useRef } from "react";
import { TransformWrapper, TransformComponent } from "react-zoom-pan-pinch";
import { useEditorStore } from "../store/use-editor-store";
import { EditorElement } from "./editor-element";
import { SelectionManager } from "./selection-manager";
import { EditorContextMenu } from "./context-menu";

export function Canvas() {
  const { elements, canvasConfig, setCanvasConfig } = useEditorStore();
  const workspaceRef = useRef<HTMLDivElement>(null);
  const [isMiddleMouseDown, setIsMiddleMouseDown] = React.useState(false);

  return (
    <div 
      className="flex-1 relative overflow-hidden" 
      style={{ backgroundColor: canvasConfig.backgroundColor || "#121212" }}
    >
      <TransformWrapper
        initialScale={1}
        minScale={0.05}
        maxScale={10}
        centerOnInit={true}
        onZoom={(ref) => setCanvasConfig({ zoom: ref.state.scale })}
        panning={{
          excluded: ["input", "button"],
          activationKeys: isMiddleMouseDown ? [] : [" "], 
        }}
        limitToBounds={false}
        doubleClick={{ disabled: true }}
        wheel={{ step: 0.1, wheelDisabled: false }}
      >
        <div 
          className="w-full h-full"
          onMouseDown={(e) => {
            if (e.button === 1) {
              setIsMiddleMouseDown(true);
            }
          }}
          onMouseUp={(e) => {
             if (e.button === 1) {
               setIsMiddleMouseDown(false);
             }
          }}
          onClick={(e) => {
             const target = e.target as HTMLElement;
             const isElement = target.closest(".editor-element-container");
             const isMoveable = target.closest(".moveable-control-box");
             
             if (!isElement && !isMoveable) {
               useEditorStore.getState().setSelectedElements([]);
             }
          }}
        >
          <TransformComponent
              wrapperStyle={{ width: "100%", height: "100%", cursor: "default" }}
              contentStyle={{
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                minWidth: "100%",
                minHeight: "100%",
                padding: "5000px", 
              }}
            >
              <EditorContextMenu>
            <div
                ref={workspaceRef}
                id="editor-workspace"
                className="relative"
                style={{
                  width: "100px",
                  height: "100px",
                  pointerEvents: "auto",
                }}
            >
                {/* Elements (including Frames) */}
                {elements.map((element) => (
                <EditorElement key={element.id} element={element} />
                ))}
                
                <SelectionManager containerRef={workspaceRef} />
            </div>
          </EditorContextMenu>
        </TransformComponent>
      </div>
    </TransformWrapper>
   </div>
  );
}
