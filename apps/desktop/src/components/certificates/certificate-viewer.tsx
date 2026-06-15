import { cn } from "@orbis/ui/cn";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { EditorElement as IEditorElement } from "@/types/certificate-editor";
import { GradesTable } from "./grades-table";

interface CertificateViewerProps {
  template: {
    backgroundImageUrl?: string | null;
    content?:
      | {
          elements: IEditorElement[];
          layoutConfig?: {
            rows: number;
            columns: number;
            gap: number;
          };
        }
      | any;
    size?: string;
    orientation?: string;
  };
  data?:
    | Record<string, string | number>
    | Array<Record<string, string | number>>;
  certificateType?: string;
  month?: number;
  preview?: boolean;
  className?: string;
  onLoad?: () => void;
}

export function CertificateViewer({
  template,
  data = {},
  certificateType,
  month,
  preview = false,
  className,
  onLoad,
}: CertificateViewerProps) {
  const elements = useMemo(() => {
    const rawElements = (template.content?.elements || []) as IEditorElement[];
    return rawElements.sort((a, b) => a.zIndex - b.zIndex);
  }, [template]);

  const canvasConfig = template.content?.canvasConfig;
  const layoutConfig = canvasConfig?.layoutConfig ||
    template.content?.layoutConfig || { rows: 1, columns: 1, gap: 0 };

  // State for asset tracking
  const [assetsReady, setAssetsReady] = useState(false);
  const loadingRegistry = useRef<Set<string>>(new Set());
  const onLoadCalledRef = useRef(false);

  // Normalize data for grid rendering
  const dataArray = useMemo(
    () => (Array.isArray(data) ? data : [data]),
    [data],
  );
  const totalSlots = layoutConfig.rows * layoutConfig.columns;
  const activeData = useMemo(
    () =>
      preview
        ? [
            ...dataArray,
            ...Array(Math.max(0, totalSlots - dataArray.length)).fill({}),
          ]
        : dataArray.slice(0, totalSlots),
    [dataArray, totalSlots, preview],
  );

  // Dimensions math
  const baseFrame = useMemo(
    () => elements.find((el) => el.type === "frame"),
    [elements],
  );
  const sheetWidth = baseFrame?.width || 794;
  const sheetHeight = baseFrame?.height || 1123;
  const slotWidth =
    (sheetWidth - (layoutConfig.columns - 1) * (layoutConfig.gap || 0)) /
    layoutConfig.columns;
  const slotHeight =
    (sheetHeight - (layoutConfig.rows - 1) * (layoutConfig.gap || 0)) /
    layoutConfig.rows;

  const hasTable = useMemo(
    () => elements.some((el) => el.type === "table"),
    [elements],
  );

  // Asset synchronization logic
  const registerAsset = useCallback((id: string) => {
    loadingRegistry.current.add(id);
  }, []);

  const unregisterAsset = useCallback((id: string) => {
    loadingRegistry.current.delete(id);
    if (loadingRegistry.current.size === 0) {
      setAssetsReady(true);
    }
  }, []);

  // Pre-register all assets synchronously to avoid race conditions
  useMemo(() => {
    loadingRegistry.current.clear();

    if (template.backgroundImageUrl) {
      loadingRegistry.current.add("background");
    }

    activeData.forEach((recordData, idx) => {
      elements.forEach((el) => {
        if (
          el.type === "image" &&
          (el.variableKey ? recordData[el.variableKey] : el.content)
        ) {
          loadingRegistry.current.add(`el-${idx}-${el.id}`);
        }
      });
    });

    if (loadingRegistry.current.size === 0) {
      setAssetsReady(true);
    } else {
      setAssetsReady(false);
    }
  }, [activeData, elements, template.backgroundImageUrl]);

  // Effect to handle final readiness signal
  useEffect(() => {
    if (onLoadCalledRef.current) return;

    if (assetsReady) {
      const triggerReady = () => {
        if (onLoadCalledRef.current) return;
        onLoadCalledRef.current = true;
        console.log("[CertificateViewer] Assets ready, triggering onLoad");
        onLoad?.();
      };

      // Extra safety buffer for complex components (Tables/Gradients/Paints)
      const timer = setTimeout(triggerReady, hasTable ? 1200 : 500);
      return () => clearTimeout(timer);
    }
  }, [assetsReady, onLoad, hasTable]);

  const renderSingleRecord = (
    recordData: Record<string, any>,
    recordIndex: number,
  ) => {
    const row = Math.floor(recordIndex / layoutConfig.columns);
    const col = recordIndex % layoutConfig.columns;
    const offsetX = col * (slotWidth + (layoutConfig.gap || 0));
    const offsetY = row * (slotHeight + (layoutConfig.gap || 0));

    if (!recordData || (Object.keys(recordData).length === 0 && !preview))
      return null;

    return (
      <div
        key={`record-${recordIndex}-${recordData.studentId || recordIndex}`}
        className="absolute overflow-hidden select-none"
        style={{
          left: `${offsetX}px`,
          top: `${offsetY}px`,
          width: `${slotWidth}px`,
          height: `${slotHeight}px`,
        }}
      >
        {elements.map((el) => {
          if (!el.visible || el.id === baseFrame?.id) return null;

          const style = el.style || {};
          const left = baseFrame ? el.x - baseFrame.x : el.x;
          const top = baseFrame ? el.y - baseFrame.y : el.y;

          let content = el.content || "";
          if (el.type === "variable" && el.variableKey) {
            content = String(
              recordData[el.variableKey] || (preview ? `{${el.name}}` : ""),
            );
          }

          const elementId = `el-${recordIndex}-${el.id}`;

          return (
            <div
              key={elementId}
              className={cn(
                "absolute flex transition-opacity duration-300",
                style.textAlign === "center"
                  ? "justify-center"
                  : style.textAlign === "right"
                    ? "justify-end"
                    : "justify-start",
              )}
              style={{
                left: `${left}px`,
                top: `${top}px`,
                width: `${el.width}px`,
                height: `${el.height}px`,
                transform: `rotate(${el.rotation || 0}deg)`,
                opacity: el.opacity ?? 1,
                zIndex: el.zIndex,
                pointerEvents: "none",
              }}
            >
              {el.type === "image" ? (
                <ImageAsset
                  id={elementId}
                  src={el.variableKey ? recordData[el.variableKey] : el.content}
                  alt={el.name}
                  onRegister={registerAsset}
                  onUnregister={unregisterAsset}
                />
              ) : el.type === "shape" ? (
                <div
                  className="w-full h-full"
                  style={{
                    backgroundColor: style.backgroundColor,
                    borderRadius: `${style.borderRadius || 0}px`,
                    border: `${style.borderWidth || 0}px solid ${style.borderColor || "transparent"}`,
                    boxShadow: style.shadow || "none",
                  }}
                />
              ) : el.type === "table" ? (
                <GradesTable
                  studentId={recordData.studentId as string}
                  type={certificateType}
                  month={month}
                  isPreview={preview}
                  onLoad={() => {}} // Internal table loading is handled by the 800ms buffer
                  fontSize={style.fontSize || 10}
                />
              ) : (
                <div
                  className="w-full h-full"
                  style={{
                    fontSize: `${style.fontSize}px`,
                    fontWeight: style.fontWeight || "bold",
                    color: style.color || "#000000",
                    fontFamily: style.fontFamily,
                    textAlign: style.textAlign || "center",
                    lineHeight: style.lineHeight || 1.2,
                    whiteSpace: "pre-wrap",
                    wordBreak: "break-word",
                  }}
                >
                  {content}
                </div>
              )}
            </div>
          );
        })}
      </div>
    );
  };

  return (
    <div
      className={cn(
        "relative bg-white print:shadow-none overflow-hidden",
        className,
      )}
      style={{
        width: `${sheetWidth}px`,
        height: `${sheetHeight}px`,
        direction: "rtl",
      }}
      data-canvas-content
    >
      {template.backgroundImageUrl && (
        <img
          src={normalizeImageUrl(template.backgroundImageUrl)}
          alt="Background"
          className="w-full h-full object-fill absolute inset-0 pointer-events-none"
          onLoad={() => unregisterAsset("background")}
          onError={() => unregisterAsset("background")}
          style={{ zIndex: -1 }}
        />
      )}

      {activeData.map((recordData, idx) => renderSingleRecord(recordData, idx))}
    </div>
  );
}

import { getFileProxyUrl } from "@/utils/files";

/**
 * Normalize ANY image URL to go through the local API proxy.
 */
function normalizeImageUrl(src: string | undefined | null): string {
  if (!src) return "";
  const url = String(src).trim();
  if (!url) return "";

  if (url.startsWith("data:")) return url;

  return getFileProxyUrl(url);
}

/**
 * 2026 Optimized Image Component
 */
function ImageAsset({ id, src, alt, onRegister, onUnregister }: any) {
  useEffect(() => {
    return () => onUnregister(id);
  }, [id, src, onUnregister]);

  if (!src) return null;

  const finalSrc = useMemo(() => normalizeImageUrl(src), [src]);

  if (!finalSrc) return null;

  return (
    <img
      src={finalSrc}
      className="w-full h-full object-contain"
      alt={alt}
      crossOrigin="anonymous"
      loading="eager"
      onLoad={async (e) => {
        const img = e.currentTarget;
        if (img.decode) {
          try {
            await img.decode();
          } catch (err) {
            console.warn(`[ImageAsset] Decode failed for ${id}:`, err);
          }
        }
        onUnregister(id);
      }}
      onError={() => {
        console.error(`[ImageAsset] FAILED to load image from: ${finalSrc}`);
        onUnregister(id);
      }}
    />
  );
}
